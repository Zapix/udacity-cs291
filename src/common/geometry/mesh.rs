use std::borrow::Cow;

use wgpu::{Color};
use wgpu::util::DeviceExt;

use crate::common::geometry::geometry::Geometry;
use crate::common::vertex::Vertex;

pub struct Mesh {
    geometry: Geometry,
    color: Color,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
}

impl Mesh {
    pub fn new(device: &wgpu::Device, geometry: Geometry, color: Color) -> Self {
        let vertex_buffer = Self::vertex_buffer(
            &device,
            &Self::vertecies(&geometry, &color)
        );

        let index_buffer = Self::index_buffer(
            &device,
            &Self::indicies(&geometry)
        );

        Self {
            geometry,
            color,
            vertex_buffer,
            index_buffer,
        }
    }


    pub fn create_render_pipeline(
        device: &wgpu::Device,
        surface_format: &wgpu::TextureFormat,
    ) -> wgpu::RenderPipeline {
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("render pipeline layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
        });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Mesh pipeline layout"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
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

    fn vertecies<'a>(geometry: &'a Geometry, color: &'a Color) -> Vec<Vertex> {
        geometry.verticies
            .iter()
            .map(|x| Vertex::from_point(x, color))
            .collect::<Vec<Vertex>>()
    }

    fn vertex_buffer(device: &wgpu::Device, vertecies: &[Vertex]) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Mesh vertex buffer"),
            contents: bytemuck::cast_slice(vertecies),
            usage: wgpu::BufferUsages::VERTEX,
        })
    }

    pub fn indicies<'a>(geometry: &'a Geometry) -> Vec<u16> {
        geometry.faces
            .iter()
            .map(|x| x.as_array())
            .flatten()
            .collect::<Vec<u16>>()
    }

    fn index_buffer(device: &wgpu::Device, indicies: &[u16]) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Mesh index buffer"),
            contents: bytemuck::cast_slice(indicies),
            usage: wgpu::BufferUsages::INDEX
        })
    }

    pub fn index_len(&self) -> usize {
        self.geometry.faces.len() * 3
    }
}

pub trait DrawMesh<'a> {
    fn draw_mesh(&mut self, mesh: &'a Mesh);
}

impl<'a, 'b> DrawMesh<'b> for wgpu::RenderPass<'a> where 'b : 'a {
    fn draw_mesh(&mut self, mesh: &'b Mesh) {
        self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        self.draw_indexed(0..mesh.index_len() as u32, 0, 0..1);
    }
}