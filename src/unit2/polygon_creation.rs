use std::rc::Rc;
use std::cell::RefCell;
use std::f32::consts::PI;
use web_sys::HtmlCanvasElement;

use crate::common::animation_frame::redraw_on_animation_frame;
use crate::common::canvas_unit_trait::CanvasUnitTrait;
use crate::common::flat_grid_canvas_renderer::FlatGridCanvasRenderer;
use crate::common::geometry::mesh::Mesh;
use crate::common::geometry::face3::Face3;
use crate::common::geometry::geometry::Geometry;
use crate::common::geometry::point::Point;
use crate::common::unit_trait::{UnitIdentifierTrait, UnitTrait};

const CELL_SIZE: u32 = 128;

fn polygon_creation(device: &wgpu::Device, sides: u32) -> Mesh {
    let mut geometry = Geometry::new();

    for i in 0..sides {
        let angle = (PI / 2.0) + (i as f32 / sides as f32) * 2.0 * PI;

        let x = angle.cos();
        let y = angle.sin();

        geometry.verticies.push(Point::new(x, y, 0.0));
    }

    for i in 0..(sides - 2) {
        geometry.faces.push(Face3::new(0, (i + 1) as u16, (i + 2) as u16));
    }

    Mesh::new(&device, geometry, wgpu::Color::RED)
}

fn get_meshes(device: &wgpu::Device) -> Vec<Mesh> {
    vec![
        polygon_creation(&device, 6)
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

pub struct PolygonCreation {}

impl UnitIdentifierTrait for PolygonCreation {
    fn new() -> Self {
        Self {}
    }

    fn identifier(&self) -> String {
        String::from("polygon_creation")
    }

    fn label(&self) -> String {
        String::from("Lesson 2: Polygon Creation")
    }
}

impl CanvasUnitTrait for PolygonCreation {
    fn draw_canvas(&self, canvas: HtmlCanvasElement, canvas_unmounted: Rc<RefCell<bool>>) {
        wasm_bindgen_futures::spawn_local(
            start_wgpu_with_request_animation_frame(canvas, canvas_unmounted)
        );
    }
}

impl UnitTrait for PolygonCreation {}