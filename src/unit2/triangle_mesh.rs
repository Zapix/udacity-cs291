use std::cell::RefCell;
use std::rc::Rc;
use web_sys::{HtmlCanvasElement};

use crate::common::canvas_unit_trait::CanvasUnitTrait;
use crate::common::geometry::mesh::{Mesh};
use crate::common::geometry::geometry::Geometry;
use crate::common::geometry::face3::Face3;
use crate::common::geometry::point::Point;
use crate::common::flat_grid_canvas_renderer::FlatGridCanvasRenderer;
use crate::common::animation_frame::redraw_on_animation_frame;

use crate::common::unit_trait::{UnitTrait, UnitIdentifierTrait};


const CELL_SIZE: u32 = 64;

fn draw_triangle(device: &wgpu::Device) -> Mesh {
    // This code demonstrates how to draw a triangle
    let mut geometry = Geometry::new();
    geometry.verticies.push(Point::new(0.0, 1.0, 0.0));
    geometry.verticies.push(Point::new(-1.0, -1.0, 0.0));
    geometry.verticies.push(Point::new(1.0, -1.0, 0.0));

    geometry.faces.push(Face3::new(0, 1, 2));

    Mesh::new(&device, geometry, wgpu::Color::BLUE)
}

fn draw_square(device: &wgpu::Device) -> Mesh {
    // Your code goes here

    let mut rectangle_geometry = Geometry::new();
    rectangle_geometry.verticies.push(Point::new(2.0, 5.0, 0.0));
    rectangle_geometry.verticies.push(Point::new(2.0, 2.0, 0.0));
    rectangle_geometry.verticies.push(Point::new(6.0, 2.0, 0.0));
    rectangle_geometry.verticies.push(Point::new(6.0, 5.0, 0.0));

    rectangle_geometry.faces.push(Face3::new(0, 1, 2));
    rectangle_geometry.faces.push(Face3::new(3, 0, 2));

    return Mesh::new(
        &device,
        rectangle_geometry,
        wgpu::Color {
            r: 0.9647,
            g: 0.5137,
            b: 0.1176,
            a: 1.0,
        }
    )
}

fn get_meshes(device: &wgpu::Device) -> Vec<Mesh> {
    vec![
        draw_triangle(&device),
        draw_square(&device),
    ]
}

async fn start_wgpu_with_request_animation_frame(canvas: HtmlCanvasElement, canvas_unmounted: Rc<RefCell<bool>>)  {
    let canvas_renderer = FlatGridCanvasRenderer::new(
        canvas,
        CELL_SIZE as f32,
        Box::new(get_meshes)
    ).await;
    redraw_on_animation_frame(canvas_renderer, canvas_unmounted.clone()) ;
}

pub struct TriangleMesh {}

impl UnitIdentifierTrait for TriangleMesh {
    fn new() -> Self {
        Self {}
    }

    fn identifier(&self) -> String {
        String::from("triangle_mesh")
    }

    fn label(&self) -> String {
        String::from("Lesson 2: Triangle mesh")
    }
}

impl CanvasUnitTrait for TriangleMesh {
    fn draw_canvas(&self, canvas: HtmlCanvasElement, canvas_unmounted: Rc<RefCell<bool>>) {
        wasm_bindgen_futures::spawn_local(
            start_wgpu_with_request_animation_frame(canvas, canvas_unmounted)
        );
    }
}

impl UnitTrait for TriangleMesh {}