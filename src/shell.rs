use crate::framework::tracking;
use sauron::{Application, Cmd, MouseEvent, Node, html};

use crate::shapes::app;

// The structure of the code is based on The Elm Architecture (TEA) as
// interpreted by the Sauron Framework. Naming (Model, Msg, update, view) is
// standard.
// https://guide.elm-lang.org/architecture/

// The model consists of the app and the tracker information (if any)
pub struct Model {
    app: app::Model,
}

impl Model {
    // Create a new demo model
    pub fn new() -> Self {
        Self {
            app: app::Model::new(),
        }
    }
}

pub enum Msg {
    // Interface back to the tracker.
    ToApp(app::Msg),
    // Mouse events for the tracker.
    FromTracking(tracking::Event),
}

//---- Message helpers

impl Msg {
    // Convert a mouse move event into a Msg.
    fn track_mouse_move(evt: MouseEvent) -> Self {
        evt.stop_propagation();
        Self::FromTracking(tracking::Event::mouse_move(evt))
    }

    // Convert a mouse up event into a Msg
    fn track_mouse_up(evt: MouseEvent) -> Self {
        evt.stop_propagation();
        Self::FromTracking(tracking::Event::mouse_up(evt))
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
            FromTracking(evt) => self.update_app(&app::Msg::from_tracking(evt)),
            ToApp(app_msg) => self.update_app(app_msg),
        }
    }
}

impl Model {
    fn update_app(&mut self, app_msg: &app::Msg) -> Cmd<Msg> {
        self.app.update(app_msg);
        Cmd::none()
    }
}
