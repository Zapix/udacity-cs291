use wgpu;
use crate::common::geometry::point::Point;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 4],
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
        0 => Float32x3,
        1 => Float32x4
    ];
    pub fn new(x: f32, y: f32, z: f32, color: wgpu::Color) -> Self {
        Self {
            position: [x, y, z],
            color: [color.r as f32, color.g as f32, color.b as f32, color.a as f32],
        }
    }

    pub fn from_point(point: &Point, color: &wgpu::Color) -> Self {
        Self {
            position: point.position(),
            color: [color.r as f32, color.g as f32, color.b as f32, color.a as f32],
        }
    }

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}
