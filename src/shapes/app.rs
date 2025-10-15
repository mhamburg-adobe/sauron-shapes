// This code covers the App State. In this case, it consists of the
// shapes document and the information for coloring new shapes.

use sauron::{MouseEvent, Node, events, id, svg};

use crate::framework::tracking;
use crate::shapes::core::{Color, Geometry, Shape, Style, XYPoint};
use crate::shapes::doc::{Document, ShapeId};

use std::vec::Vec;

// Our model is simple. It consists of the document being edited and
// information for tools. At this point, the latter just contains
// the fill color for new shapes.
pub struct Model {
    doc: Document,
    fill_color: Color,
    tracking_state: TrackingState,
}

// Messages we can use to update the model.
pub enum Msg {
    // A mouse down event on a shape.
    ShapeMouseDown(ShapeId, MouseEvent),
    // A mouse down on the background.
    BackgroundMouseDown(MouseEvent),
    // Tracking event
    FromTracking(tracking::Event),
}

impl Msg {
    pub fn from_tracking(tracking_event: &tracking::Event) -> Self {
        Self::FromTracking(tracking_event.clone())
    }
}

impl Model {
    // Generate a new document. At this point, we wire it to the demo
    // case but in the future, this will likely just generate an empty
    // document.
    pub fn new() -> Self {
        Self::new_demo()
    }

    pub fn update(&mut self, msg: &Msg) {
        use Msg::*;
        match msg {
            ShapeMouseDown(shape_id, mouse_down) => {
                DragShape::start(self, shape_id, mouse_down);
            }

            BackgroundMouseDown(mouse_down) => {
                DragNewRect::start(self, mouse_down);
                // Advance the fill color skipping white. This is purely
                // part of the demo logic to make shape drawing more
                // interesting.
                loop {
                    self.fill_color.advance();
                    if self.fill_color != Color::White {
                        break;
                    }
                }
            }

            FromTracking(tracking_event) => {
                // Clone so that we do not get an alias conflict over the
                // tracking state.
                self.tracking_state
                    .clone()
                    .update_model_for_tracking_event(self, tracking_event)
            }
        }
    }

    pub fn view(&self) -> Node<Msg> {
        use svg::attributes::*;
        use svg::*;

        let shape_count = self.doc.shape_ids_sequence_iter().count();

        let mut children = Vec::with_capacity(shape_count + 1);

        // Add the background

        children.push(background());

        // Add the shapes to the children

        children.extend(
            self.doc
                .shape_id_shapes_iter()
                .map(|(shape_id, shape)| render_shape(shape_id, shape)),
        );

        svg(
            [
                width("100%"),
                height("1500px"),
                preserve_aspect_ratio("none"),
            ],
            children,
        )
    }
}

// Render a shape to SVG and attach a mouse down handler that
// initiates dragging.

fn render_shape(shape_id: ShapeId, shape: &Shape) -> Node<Msg> {
    use svg::attributes::*;
    use svg::*;

    let id_string = format!("shape_{shape_id}");
    let fill_color = svg_color(&shape.style.fill);

    match &shape.geometry {
        Geometry::Circle { center, radius } => circle(
            vec![
                id(id_string),
                cx(center.x),
                cy(center.y),
                r(*radius),
                fill(fill_color),
                events::on_mousedown(move |evt| shape_mouse_down(shape_id, evt)),
            ],
            [],
        ),
        Geometry::Rectangle { top_left, size } => rect(
            vec![
                id(id_string),
                x(top_left.x),
                y(top_left.y),
                width(size.x),
                height(size.y),
                fill(fill_color),
                events::on_mousedown(move |evt| shape_mouse_down(shape_id, evt)),
            ],
            [],
        ),
    }
}

fn shape_mouse_down(shape_id: ShapeId, evt: MouseEvent) -> Msg {
    evt.stop_propagation();
    Msg::ShapeMouseDown(shape_id, evt)
}

fn background() -> Node<Msg> {
    use svg::attributes::*;
    use svg::*;

    rect(
        vec![
            id("background"),
            x("0"),
            y("0"),
            width("100%"),
            height("100%"),
            fill("white"),
            events::on_mousedown(background_mouse_down),
        ],
        [],
    )
}

fn svg_color(color: &Color) -> String {
    match color {
        Color::Red => "red".to_string(),
        Color::Orange => "orange".to_string(),
        Color::Yellow => "yellow".to_string(),
        Color::Green => "green".to_string(),
        Color::Blue => "blue".to_string(),
        Color::Indigo => "indigo".to_string(),
        Color::Violet => "violet".to_string(),
        Color::White => "white".to_string(),
        Color::Black => "black".to_string(),
    }
}

fn background_mouse_down(evt: MouseEvent) -> Msg {
    evt.stop_propagation();
    Msg::BackgroundMouseDown(evt)
}

impl Model {
    // Create a new demo model
    fn new_demo() -> Self {
        Self {
            doc: Document::new_demo(),
            fill_color: Color::Red,
            tracking_state: TrackingState::None,
        }
    }

    // Get the style for new shapes
    fn get_new_shape_style(&self) -> Style {
        let fill_color = self.fill_color.clone();
        Style::new(fill_color)
    }

    // Upsert a shape
    // https://en.wiktionary.org/wiki/upsert
    fn upsert_shape_with_id(&mut self, shape_id: &ShapeId, new_shape: Shape) {
        self.doc.upsert_shape_with_id(shape_id, new_shape);
    }

    // Delete a shape if it exists. Do nothing if it does not.
    fn delete_shape_with_id(&mut self, shape_id: &ShapeId) {
        self.doc.delete_shape_with_id(shape_id);
    }

    // Replace the geometry of a shape
    fn set_geometry_for_shape_with_id(&mut self, shape_id: &ShapeId, new_geometry: Geometry) {
        self.doc.set_geometry_for_shape_id(shape_id, new_geometry);
    }

    // Generate a new shape id
    fn generate_shape_id(&mut self) -> ShapeId {
        self.doc.generate_shape_id()
    }

    // Set the tracking state
    fn set_tracking_state(&mut self, new_tracking_state: TrackingState) {
        self.tracking_state = new_tracking_state;
    }

    // Clear the tracking state

    fn stop_tracking(&mut self) {
        self.tracking_state = TrackingState::None;
    }
}

// Tracking

// When we are tracking the mouse, we have a tracking state
// that is used to hold the information about the type of
// tracking we are doing.

#[derive(Clone)]
enum TrackingState {
    None,
    DragNewRect(DragNewRect),
    DragShape(DragShape),
}

// Dispatch updates based on the tracking state
impl TrackingState {
    fn update_model_for_tracking_event(&self, model: &mut Model, tracking_event: &tracking::Event) {
        match self {
            TrackingState::None => {}
            TrackingState::DragNewRect(drag_new_rect) => {
                drag_new_rect.update_model_for_tracking_event(model, tracking_event)
            }
            TrackingState::DragShape(drag_shape) => {
                drag_shape.update_model_for_tracking_event(model, tracking_event)
            }
        }
    }
}

#[derive(Clone)]
struct DragNewRect {
    shape_id: ShapeId,
    mouse_down_position: XYPoint,
    style: Style,
}

impl DragNewRect {
    fn start(model: &mut Model, mouse_down: &MouseEvent) {
        let shape_id = model.generate_shape_id();
        let style = model.get_new_shape_style();
        model.set_tracking_state(TrackingState::DragNewRect(Self {
            shape_id,
            style,
            mouse_down_position: get_page_coordinates(mouse_down),
        }))
    }

    fn update_model_for_tracking_event(&self, model: &mut Model, tracking_event: &tracking::Event) {
        let drag_position = get_page_coordinates(&tracking_event.mouse_event);
        let (min_x, span_x) = to_min_span(self.mouse_down_position.x, drag_position.x);
        let (min_y, span_y) = to_min_span(self.mouse_down_position.y, drag_position.y);
        // If non-empty, upsert the shape
        if 0.0 < span_x && 0.0 < span_y {
            let geometry = Geometry::Rectangle {
                top_left: XYPoint::new(min_x, min_y),
                size: XYPoint::new(span_x, span_y),
            };
            let shape = Shape {
                geometry,
                style: self.style.clone(),
            };
            model.upsert_shape_with_id(&self.shape_id, shape);
        // If empty, delete the shape.
        } else {
            model.delete_shape_with_id(&self.shape_id)
        }
        if tracking_event.selector == tracking::Selector::MouseUp {
            model.stop_tracking()
        }
    }
}

#[derive(Clone)]
struct DragShape {
    shape_id: ShapeId,
    original_geometry: Geometry,
    mouse_down_position: XYPoint,
}

impl DragShape {
    fn start(model: &mut Model, shape_id: &ShapeId, mouse_down: &MouseEvent) {
        if let Some(shape) = model.doc.get_shape_by_id(shape_id) {
            model.set_tracking_state(TrackingState::DragShape(Self {
                shape_id: *shape_id,
                original_geometry: shape.geometry.clone(),
                mouse_down_position: get_page_coordinates(mouse_down),
            }))
        }
    }

    fn update_model_for_tracking_event(&self, model: &mut Model, tracking_event: &tracking::Event) {
        let drag_position = get_page_coordinates(&tracking_event.mouse_event);
        let delta = drag_position.subtract(&self.mouse_down_position);
        model.set_geometry_for_shape_with_id(
            &self.shape_id,
            self.original_geometry.offset_by(&delta),
        );
        if tracking_event.selector == tracking::Selector::MouseUp {
            model.stop_tracking()
        }
    }
}

// Utlities

// Given a pair of coordinates, find the mimimum coordinate and the non-negative span
// to the other coordinate.

fn to_min_span(x1: f64, x2: f64) -> (f64, f64) {
    if x1 < x2 {
        (x1, x2 - x1)
    } else {
        (x2, x1 - x2)
    }
}

// Extract page coordinates from a mouse event.

pub fn get_page_coordinates(mouse_event: &MouseEvent) -> XYPoint {
    XYPoint::new(mouse_event.page_x().into(), mouse_event.page_y().into())
}
