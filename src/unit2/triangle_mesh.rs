use std::borrow::Cow;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Element, HtmlCanvasElement, console};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::event::{WindowEvent, Event};
use winit::window::Window;

#[cfg(target_arch = "wasm32")]
use winit::platform::web::EventLoopExtWebSys;
#[cfg(target_arch = "wasm32")]
use winit::platform::web::WindowBuilderExtWebSys;

use crate::common::traits::UnitTrait;

pub struct TriangleMesh {}

const CANVAS_ID: &'static str  = "canvas";


async fn start(window: Window, event_loop: EventLoop<()>) {
    let size = window.inner_size();
    let height = size.height.max(1);
    let width = size.width.max(1);

    console::log_1(&format!("Size: {}x{}", width, height).as_str().into());

    let instance = wgpu::Instance::default();

    let surface = unsafe {
        instance.create_surface(&window).unwrap()
    };

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

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });


    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(surface_format.into())],
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
    });

    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.set_control_flow(ControlFlow::Wait);


    event_loop.spawn(move |event, target| {
        let _ =(&instance, &adapter, &render_pipeline, &shader);


        match event {
            Event::WindowEvent {
                event:WindowEvent::Resized(new_size),
                ..
            } => {
                let width = new_size.width.max(1);
                let height = new_size.height.max(1);
                config.width = width;
                config.height = height;
                surface.configure(&device, &config);
            },
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
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
                                load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                                store: wgpu::StoreOp::Store,
                            },
                        })],
                        depth_stencil_attachment: None,
                        timestamp_writes: None,
                        occlusion_query_set: None,
                    });
                    render_pass.set_pipeline(&render_pipeline);
                    render_pass.draw(0..3, 0..1);
                }

                queue.submit(Some(encoder.finish()));
                frame.present();
            },
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => target.exit(),
            Event::WindowEvent {
                event: WindowEvent::Occluded(_),
                ..
            } => {
                target.exit();
            },
            _ => {}
        }
    });
}


impl UnitTrait for TriangleMesh {
    fn new() -> Self {
        Self {}
    }

    fn identifier(&self) -> String {
        String::from("triangle_mesh")
    }

    fn label(&self) -> String {
        String::from("Lesson 2: Triangle mesh")
    }

    fn render(&self, base: &Element) -> Result<(), JsValue> {
        let window = web_sys::window().expect("Window does not exist");
        let document = window.document().expect("Can get document");

        let canvas = document
            .create_element("canvas").unwrap()
            .dyn_into::<HtmlCanvasElement>().unwrap();

        canvas.set_attribute("id", CANVAS_ID).unwrap();
        canvas.set_attribute("style", "width: 100%; height: 100%").unwrap();

        base.append_child(&canvas).unwrap();

        let event_loop = EventLoop::new().unwrap();
        let mut builder = winit::window::WindowBuilder::new();
        builder = builder.with_canvas(Some(canvas));

        let window = builder.build(&event_loop).unwrap();

        wasm_bindgen_futures::spawn_local(start(window, event_loop));

        Ok(())
    }
}