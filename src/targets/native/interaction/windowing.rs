use crate::interaction;
use crate::interaction::windowing::{Event, Action};

use std::sync::{Arc, RwLock};

pub(crate) struct WindowState {
    handlers: Vec<Box<dyn Fn(Event) + Sync + Send>>,
}

pub(crate) struct Window {
    state: Arc<RwLock<WindowState>>,
}

impl interaction::Source for Window {
    type Event = Event;
    fn bind(&self, handler: Box<dyn Fn(Self::Event) + 'static + Sync + Send>) {
        self.state.write().unwrap().handlers.push(handler);
    }
}

impl interaction::Window for Window {
    //TODO
    fn set_title(&mut self, title: &'_ str) {}
}

impl Window {
    pub(crate) fn new(event_handler: Box<dyn interaction::Source<Event = glutin::Event>>) -> Box<dyn interaction::Window> {
        let window = Window {
            state: Arc::new(RwLock::new(WindowState { handlers: vec![] })),
        };
        window.initialize(event_handler);
        Box::new(window)
    }

    fn initialize(&self, event_handler: Box<dyn interaction::Source<Event = glutin::Event>>) {
        let state = self.state.clone();
        event_handler.bind(Box::new(move |event: glutin::Event| {
            let my_state = state.clone();
            let mut state = my_state.write().unwrap();
            if let glutin::Event::WindowEvent { event, .. } = event {
                let action: Option<Action> = match event {
                    glutin::WindowEvent::Resized(_) => Some(Action::Resize),
                    glutin::WindowEvent::Moved(p) => Some(Action::Move((p.x, p.y).into())),
                    _ => None,
                };
                if action.is_some() {
                    state.handlers.iter().for_each(|handler| {
                        handler(Event {
                            action: action.unwrap(),
                        })
                    })
                }
            }
        }));
    }
}
