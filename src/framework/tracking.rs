use sauron::MouseEvent;

// Labeling for tracking events. We could probably test the type
// for the event, but labeling where we got it seems better.

#[derive(Clone, PartialEq)]
pub enum Selector {
    MouseMove,
    MouseUp,
}

#[derive(Clone)]
pub struct Event {
    pub selector: Selector,
    pub mouse_event: MouseEvent,
}

impl Event {
    pub fn mouse_move(mouse_event: MouseEvent) -> Event {
        Event {
            selector: Selector::MouseMove,
            mouse_event,
        }
    }

    pub fn mouse_up(mouse_event: MouseEvent) -> Event {
        Event {
            selector: Selector::MouseUp,
            mouse_event,
        }
    }
}
