use std::rc::Rc;
use std::cell::RefCell;
use web_sys::HtmlCanvasElement;

use crate::common::animation_frame::redraw_on_animation_frame;
use crate::common::canvas_unit_trait::CanvasUnitTrait;
use crate::common::flat_grid_canvas_renderer::FlatGridCanvasRenderer;
use crate::common::geometry::mesh::Mesh;
use crate::common::geometry::face3::Face3;
use crate::common::geometry::geometry::Geometry;
use crate::common::geometry::point::Point;
use crate::common::unit_trait::{UnitIdentifierTrait, UnitTrait};

const CELL_SIZE: u32 = 64;

fn render_some_object(device: &wgpu::Device) -> Mesh {
    let mut geometry = Geometry::new();

    geometry.verticies.push(Point::new(3.0, 3.0, 0.0));
    geometry.verticies.push(Point::new(7.0, 3.0, 0.0));
    geometry.verticies.push(Point::new(7.0, 7.0, 0.0));
    geometry.verticies.push(Point::new(3.0, 7.0, 0.0));

    geometry.faces.push(Face3::new(0, 1, 2));
    geometry.faces.push(Face3::new(2, 3, 0));

    Mesh::new(&device, geometry, wgpu::Color::BLUE)
}

fn get_meshes(device: &wgpu::Device) -> Vec<Mesh> {
    vec![
        render_some_object(&device),
    ]
}

async fn start_wgpu_with_request_animation_frame(
    canvas: HtmlCanvasElement,
    canvas_unmounted: Rc<RefCell<bool>>,
) {
    let canvas_renderer = FlatGridCanvasRenderer::new(
        canvas,
        CELL_SIZE as f32,
        Box::new(get_meshes)
    ).await;

    redraw_on_animation_frame(canvas_renderer, canvas_unmounted.clone());
}

pub struct VertexOrder {}

impl UnitIdentifierTrait for VertexOrder {
    fn new() -> Self {
        Self {}
    }
    fn identifier(&self) -> String {
        String::from("vertex_order")
    }

    fn label(&self) -> String {
        String::from("Lesson 2: Vertex order")
    }

}

impl CanvasUnitTrait for VertexOrder {
    fn draw_canvas(&self, canvas: HtmlCanvasElement, canvas_unmounted: Rc<RefCell<bool>>) {
        wasm_bindgen_futures::spawn_local(
            start_wgpu_with_request_animation_frame(canvas, canvas_unmounted)
        );
    }
}

impl UnitTrait for VertexOrder {}
