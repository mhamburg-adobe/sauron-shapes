use crate::framework::tracker;
use sauron::{Application, Cmd, MouseEvent, Node, html};

use crate::shapes::app;

use tracker::Tracker;

// The structure of the code is based on The Elm Architecture (TEA) as
// interpreted by the Sauron Framework. Naming (Model, Msg, update, view) is
// standard.
// https://guide.elm-lang.org/architecture/

// The model consists of the app and the tracker information (if any)
pub struct Model {
    app: app::Model,
    opt_tracker: Option<Tracker<app::Model>>,
}

impl Model {
    // Create a new demo model
    pub fn new() -> Self {
        Self {
            app: app::Model::new(),
            opt_tracker: None,
        }
    }
}

pub enum Msg {
    // Interface back to the tracker.
    ToApp(app::Msg),
    // Mouse events for the tracker.
    TrackMouseMove(MouseEvent),
    TrackMouseUp(MouseEvent),
}

//---- Message helpers

impl Msg {
    // Convert a mouse move event into a Msg.
    fn track_mouse_move(evt: MouseEvent) -> Self {
        evt.stop_propagation();
        Self::TrackMouseMove(evt)
    }

    // Convert a mouse up event into a Msg
    fn track_mouse_up(evt: MouseEvent) -> Self {
        evt.stop_propagation();
        Self::TrackMouseUp(evt)
    }

    // Apply routing to an app message
    fn to_app(app_msg: app::Msg) -> Self {
        Self::ToApp(app_msg)
    }
}

//---- Application implementation

impl Application for Model {
    type MSG = Msg;

    fn view(&self) -> Node<Msg> {
        use html::attributes::*;
        use html::*;
        // Ugh. This consumes mouse move events even when we aren't
        // tracking.
        // Also ugh is that we really want app messages out of here
        // which fights with the containment hierarchy. Mapping and
        // unmapping this results in complaints from clippy about
        // lifetimes.
        // For the latter problem, we want to pass the children
        // to the tracker view function together with a function
        // to wrap the tracker messages. Straightforward, but then
        // we also need to make sure all the lifetime logic works
        // out.
        div(
            vec![
                class("canvas-tracker-div"),
                id("canvas-tracking"),
                events::on_mousemove(Msg::track_mouse_move),
                events::on_mouseup(Msg::track_mouse_up),
            ],
            [self.app.view().map_msg(Msg::to_app)],
        )
    }

    fn update(&mut self, msg: Msg) -> Cmd<Msg> {
        use Msg::*;
        match &msg {
            TrackMouseMove(evt) => {
                if let Some(tracker) = self.opt_tracker.clone() {
                    let next = tracker.track_mouse_move(&mut self.app, evt);
                    match next {
                        tracker::Next::Done => self.opt_tracker = None,
                        tracker::Next::Continue => {}
                        tracker::Next::Switch(new_tracker) => self.opt_tracker = Some(new_tracker),
                    }
                }
            }
            TrackMouseUp(evt) => {
                if let Some(tracker) = self.opt_tracker.clone() {
                    tracker.track_mouse_up(&mut self.app, evt);
                    self.opt_tracker = None;
                }
            }
            ToApp(app_msg) => {
                let requests = self.app.update(app_msg);
                for request in requests {
                    match request {
                        app::Request::StartTracker(new_tracker) => {
                            self.opt_tracker = Some(new_tracker)
                        }
                    }
                }
            }
        }
        Cmd::none()
    }
}
