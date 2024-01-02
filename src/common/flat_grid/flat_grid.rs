use std::borrow::Cow;
pub struct FlatGrid;

impl FlatGrid {
    pub fn create_render_pipeline(
        device: &wgpu::Device,
        surface_format: &wgpu::TextureFormat,
        resolution_bind_group_layout: &wgpu::BindGroupLayout
    ) -> wgpu::RenderPipeline {
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Grid pipeline layout"),
            bind_group_layouts: &[
                resolution_bind_group_layout,
            ],
            push_constant_ranges: &[],
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Grid shader"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
        });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Grid pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: Default::default(),
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some((*surface_format).into())],
            }),
            multiview: None,
        })
    }
}

pub trait DrawFlatGrid {
    fn draw_flat_grid(&mut self);
}

impl<'a> DrawFlatGrid for wgpu::RenderPass<'a> {
    fn draw_flat_grid(&mut self) {
        self.draw(0..6, 0..1);
    }
}