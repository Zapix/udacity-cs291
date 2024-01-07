use std::rc::{Rc};
use std::cell::{RefCell};
use wasm_bindgen::prelude::{*};
use wasm_bindgen::{JsCast};
use web_sys::{console, HtmlCanvasElement};
use wgpu::util::DeviceExt;

#[cfg(target_arch = "wasm32")]
use winit::platform::web::EventLoopExtWebSys;
use crate::common::geometry::mesh::{DrawMesh, Mesh};
use crate::common::flat_grid::flat_grid::{FlatGrid, DrawFlatGrid};
use crate::common::flat_axes::flat_axes::{FlatAxes, DrawFlatAxes};
use crate::common::geometry::face3::Face3;
use crate::common::geometry::geometry::Geometry;
use crate::common::geometry::point::Point;
use crate::common::physical_size::get_physical_size;

use crate::common::canvas_unit_trait::CanvasUnitTrait;
use crate::common::unit_trait::{UnitTrait, UnitIdentifierTrait};

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

async fn start_wgpu_with_request_animation_frame(
    canvas: HtmlCanvasElement,
    canvas_unmounted: Rc<RefCell<bool>>,
) {
    let (width, height) = get_physical_size(&canvas).expect("Can't get size of canvas");
    console::log_1(&format!("Physical size: width: {}, height: {}", canvas.client_width(), canvas.client_height()).as_str().into());

    let instance = wgpu::Instance::default();
    let surface = instance.create_surface_from_canvas(canvas).unwrap();
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: Default::default(),
        force_fallback_adapter: false,
        compatible_surface: None,
    }).await.unwrap();


    let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
        label: None,
        features: wgpu::Features::empty(),
        limits: wgpu::Limits::downlevel_webgl2_defaults(),
    }, None).await.unwrap();

    let surface_cap = surface.get_capabilities(&adapter);
    let surface_format = surface_cap.formats.iter()
        .copied()
        .filter(|x| x.is_srgb())
        .next()
        .unwrap_or(surface_cap.formats[0]);

    let mut config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width,
        height,
        present_mode: surface_cap.present_modes[0],
        alpha_mode: surface_cap.alpha_modes[0],
        view_formats: vec![]
    };
    surface.configure(&device, &config);

    let resolution_arr = [width as f32, height as f32, CELL_SIZE as f32, 0.0f32];
    let resolution_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("resolution buffer"),
        contents: bytemuck::cast_slice(&resolution_arr),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    let resolution_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }
        ],
        label: Some("resolution_bind_group_layout"),
    });

    let resolution_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("resolution_bind_group"),
        layout: &resolution_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: resolution_buffer.as_entire_binding(),
            }
        ],
    });

    let render_pipeline = Mesh::create_render_pipeline(
        &device,
        &surface_format,
        &resolution_bind_group_layout,
    );

    let shape = render_some_object(&device);

    let flat_grid_pipeline = FlatGrid::create_render_pipeline(
        &device,
        &surface_format,
        &resolution_bind_group_layout
    );

    let flat_axes_pipeline = FlatAxes::create_render_pipeline(
        &device,
        &surface_format,
        &resolution_bind_group_layout
    );

    let redraw : Rc<RefCell<Option<Closure<dyn FnMut ()>>>> = Rc::new(RefCell::new(None));
    let redraw_closure = redraw.clone();
    *redraw_closure.borrow_mut() = Some(Closure::new(move || {
        if !*canvas_unmounted.borrow() {
            let _ = redraw.borrow_mut().take();
            return;
        }

        let frame = surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor { label: None }
        );
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            render_pass.set_pipeline(&flat_grid_pipeline);
            render_pass.set_bind_group(0, &resolution_bind_group, &[]);
            render_pass.draw_flat_grid();

            render_pass.set_pipeline(&flat_axes_pipeline);
            render_pass.draw_flat_axes();

            render_pass.set_pipeline(&render_pipeline);
            render_pass.draw_mesh(&shape);
        };
        queue.submit(Some(encoder.finish()));
        frame.present();

        let window = web_sys::window().expect("Window does not exist");
        window.request_animation_frame(
            redraw.borrow().as_ref().unwrap().as_ref().unchecked_ref()
        );
    }));

    let window = web_sys::window().expect("Window does not exist");
    window.request_animation_frame(
        redraw_closure.borrow().as_ref().unwrap().as_ref().unchecked_ref()
    ).expect("requestAnemiationFrame should be available");
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

