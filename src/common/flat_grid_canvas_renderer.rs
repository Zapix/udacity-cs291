use web_sys::{console, HtmlCanvasElement};
use wgpu::util::DeviceExt;
use crate::common::flat_axes::flat_axes::{DrawFlatAxes, FlatAxes};
use crate::common::flat_grid::flat_grid::{DrawFlatGrid, FlatGrid};
use crate::common::geometry::mesh::{DrawMesh, Mesh};
use crate::common::physical_size::get_physical_size;

pub struct FlatGridCanvasRenderer {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    resolution_bind_group: wgpu::BindGroup,
    render_pipeline: wgpu::RenderPipeline,
    shapes: Vec<Mesh>,
    flat_grid_pipeline: wgpu::RenderPipeline,
    flat_axes_pipeline: wgpu::RenderPipeline,
}

impl FlatGridCanvasRenderer {
    pub async fn new(canvas: HtmlCanvasElement, cell_size: f32, get_meshes: Box<dyn Fn(&wgpu::Device) -> Vec<Mesh>>) -> Self {
        let (width, height) = get_physical_size(&canvas).expect("Can't get size of canvas");
        console::log_1(&format!("Physical size: width: {}, height: {}", width, height).as_str().into());

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

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width,
            height,
            present_mode: surface_cap.present_modes[0],
            alpha_mode: surface_cap.alpha_modes[0],
            view_formats: vec![]
        };
        surface.configure(&device, &config);

        let resolution_arr = [width as f32, height as f32, cell_size, 0.0f32];
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

        let shapes = get_meshes(&device);

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

        Self {
            surface,
            device,
            queue,
            resolution_bind_group,
            render_pipeline,
            shapes,
            flat_grid_pipeline,
            flat_axes_pipeline,
        }
    }

    pub fn redraw(&self) {
        let frame = self.surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(
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
            render_pass.set_pipeline(&self.flat_grid_pipeline);
            render_pass.set_bind_group(0, &self.resolution_bind_group, &[]);
            render_pass.draw_flat_grid();

            render_pass.set_pipeline(&self.flat_axes_pipeline);
            render_pass.draw_flat_axes();

            render_pass.set_pipeline(&self.render_pipeline);
            for shape in &*self.shapes {
                render_pass.draw_mesh(shape);
            }
        };
        self.queue.submit(Some(encoder.finish()));
        frame.present();

    }
}
