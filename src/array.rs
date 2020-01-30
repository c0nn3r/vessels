use crate::{Channels, Context, Protocol};
use futures::future::{ready, Ready};
use void::Void;

impl<C: Context, T> Protocol<C> for [T; 0] {
    type Unravel = Void;
    type UnravelError = Void;
    type UnravelFuture = Ready<Result<(), Void>>;
    type Coalesce = Void;
    type CoalesceError = Void;
    type CoalesceFuture = Ready<Result<[T; 0], Void>>;

    fn unravel(self, _: C::Unravel) -> Self::UnravelFuture
    where
        C: Channels<Self::Unravel, Self::Coalesce>,
    {
        ready(Ok(()))
    }

    fn coalesce(_: C::Coalesce) -> Self::CoalesceFuture
    where
        C: Channels<Self::Unravel, Self::Coalesce>,
    {
        ready(Ok([]))
    }
}
