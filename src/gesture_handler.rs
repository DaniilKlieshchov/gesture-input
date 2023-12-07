use crate::{configuration::Gesture, gesture_listener::GestureListener};
use input::event::gesture::*;

pub struct GestureEventHandler {
    listeners: Vec<Box<dyn GestureListener>>,
}

impl GestureEventHandler {
    pub fn new(gestures: Vec<Gesture>) -> Self {
        let listeners: Vec<Box<dyn GestureListener>> =
            gestures.into_iter().map(Gesture::listen).collect();
        Self { listeners }
    }

    pub fn process_event(&mut self, event: GestureEvent) {
        for listener in self.listeners.iter_mut() {
            listener.fire(&event);
        }
    }
}
