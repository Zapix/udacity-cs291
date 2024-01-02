use std::borrow::Cow;
pub struct FlatAxes;

impl FlatAxes {
    pub fn create_render_pipeline(
        device: &wgpu::Device,
        surface_format: &wgpu::TextureFormat,
        resolution_bind_group_layout: &wgpu::BindGroupLayout
    ) -> wgpu::RenderPipeline {
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Flat axes pipeline layout"),
            bind_group_layouts: &[resolution_bind_group_layout],
            push_constant_ranges: &[],
        });
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Flat Axes Shader"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
        });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Flat axes render pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some((*surface_format).into())],
            }),
            primitive: Default::default(),
            depth_stencil: None,
            multisample: Default::default(),
            multiview: None,
        })
    }
}

pub trait DrawFlatAxes {
    fn draw_flat_axes(&mut self);
}

impl<'a> DrawFlatAxes for wgpu::RenderPass<'a> {
    fn draw_flat_axes(&mut self) {
        self.draw(0..18, 0..1);
    }
}