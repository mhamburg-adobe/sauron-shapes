use sauron::MouseEvent;
use std::rc::Rc;

// Trackers provide the hooks for grabbing mouse move and mouse up events and redirecting
// them to a target together with some tracker specific logic. To implement a tracker,
// implement a type that implements Interface below.

// A packaged tracker.
#[derive(Clone)]
pub struct Tracker<T> {
    tracker: Rc<dyn Interface<T>>,
}

// Utility methods.
impl<T> Tracker<T> {
    pub fn new(tracker: Rc<dyn Interface<T>>) -> Self {
        Self { tracker }
    }

    pub fn track_mouse_move(&self, target: &mut T, mouse_event: &MouseEvent) -> Next<T> {
        self.tracker.track_mouse_move(target, mouse_event)
    }

    pub fn track_mouse_up(&self, target: &mut T, mouse_event: &MouseEvent) {
        self.tracker.track_mouse_up(target, mouse_event)
    }
}

// A tracker with target of type T. This handles mouse move and mouse up
// tracking.
pub trait Interface<T> {
    fn track_mouse_move(&self, target: &mut T, mouse_event: &MouseEvent) -> Next<T>;
    fn track_mouse_up(&self, target: &mut T, mouse_event: &MouseEvent);
}

// When tracking mouse move events, we can indicate whether or not we are done.
// We also allow switching trackers.

pub enum Next<T> {
    // Stop tracking
    Done,
    // Keep using the same tracker
    Continue,
    // Switch to a different tracker
    Switch(Tracker<T>),
}
