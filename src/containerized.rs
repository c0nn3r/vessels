use core::{
    convert::Infallible,
    pin::Pin,
    task::{Context, Poll},
};
use core_futures_io::{AsyncRead, AsyncWrite};
use futures::task::AtomicWaker;
use std::sync::Arc;

#[macro_export]
macro_rules! export {
    ($call:ident) => {
        #[no_mangle]
        pub extern "C" fn _vessel_entry() {
            $call($crate::_vessel_entry_construct());
        }
    };
    ($items:pat => $block:block) => {
        #[no_mangle]
        pub extern "C" fn _vessel_entry() {
            let $items = $crate::_vessel_entry_construct();
            {
                $block
            };
        }
    };
}

pub struct VesselEntry {
    pub reader: VesselReader,
    pub writer: VesselWriter,
}

pub fn _vessel_entry_construct() -> VesselEntry {
    VesselEntry {
        reader: VesselReader(()),
        writer: VesselWriter(()),
    }
}

thread_local! {
    static READER_WAKER: Arc<AtomicWaker> = Arc::new(AtomicWaker::new());
    static WRITER_WRITE_WAKER: Arc<AtomicWaker> = Arc::new(AtomicWaker::new());
    static WRITER_FLUSH_WAKER: Arc<AtomicWaker> = Arc::new(AtomicWaker::new());
    static WRITER_CLOSE_WAKER: Arc<AtomicWaker> = Arc::new(AtomicWaker::new());
}

extern "C" {
    fn _vessel_poll_read(ptr: *mut u8, len: usize) -> usize;
    fn _vessel_poll_write(ptr: *const u8, len: usize) -> usize;
    fn _vessel_poll_flush() -> usize;
    fn _vessel_poll_close() -> usize;
}

#[no_mangle]
pub extern "C" fn _vessel_wake_reader() {
    READER_WAKER.with(|waker| waker.wake())
}

#[no_mangle]
pub extern "C" fn _vessel_wake_writer_write() {
    WRITER_WRITE_WAKER.with(|waker| waker.wake())
}

#[no_mangle]
pub extern "C" fn _vessel_wake_writer_flush() {
    WRITER_FLUSH_WAKER.with(|waker| waker.wake())
}

#[no_mangle]
pub extern "C" fn _vessel_wake_writer_close() {
    WRITER_CLOSE_WAKER.with(|waker| waker.wake())
}

pub struct VesselReader(());

pub struct VesselWriter(());

impl AsyncRead for VesselReader {
    type Error = Infallible;

    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &mut [u8],
    ) -> Poll<Result<usize, Infallible>> {
        match unsafe { _vessel_poll_read(buf.as_mut_ptr(), buf.len()) } {
            0 => {
                READER_WAKER.with(|waker| waker.register(cx.waker()));
                return Poll::Pending;
            }
            n => Poll::Ready(Ok(n - 1)),
        }
    }
}

impl AsyncWrite for VesselWriter {
    type WriteError = Infallible;
    type FlushError = Infallible;
    type CloseError = Infallible;

    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &[u8],
    ) -> Poll<Result<usize, Infallible>> {
        match unsafe { _vessel_poll_write(buf.as_ptr(), buf.len()) } {
            0 => {
                WRITER_WRITE_WAKER.with(|waker| waker.register(cx.waker()));
                return Poll::Pending;
            }
            n => Poll::Ready(Ok(n - 1)),
        }
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Infallible>> {
        match unsafe { _vessel_poll_flush() } {
            0 => {
                WRITER_FLUSH_WAKER.with(|waker| waker.register(cx.waker()));
                return Poll::Pending;
            }
            1 => Poll::Ready(Ok(())),
            _ => panic!(),
        }
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Infallible>> {
        match unsafe { _vessel_poll_close() } {
            0 => {
                WRITER_CLOSE_WAKER.with(|waker| waker.register(cx.waker()));
                return Poll::Pending;
            }
            1 => Poll::Ready(Ok(())),
            _ => panic!(),
        }
    }
}
