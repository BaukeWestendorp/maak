use bevy_ecs::{component::Component, resource::Resource};
use wgpu::util::DeviceExt as _;

use crate::{
    asset::AssetHandle,
    plugin::render::{Material, WgpuHandles},
};

#[derive(Debug, Resource)]
pub struct Mesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_count: u32,
}

impl Mesh {
    pub fn cube(WgpuHandles { device, .. }: &WgpuHandles) -> Self {
        #[rustfmt::skip]
       let vertices: &[f32] = &[
            -0.5, -0.5, -0.5,
             0.5, -0.5, -0.5,
             0.5,  0.5, -0.5,
            -0.5,  0.5, -0.5,
            -0.5, -0.5,  0.5,
             0.5, -0.5,  0.5,
             0.5,  0.5,  0.5,
            -0.5,  0.5,  0.5,
        ];

        #[rustfmt::skip]
        let indices: &[u16] = &[
            0, 1, 2, 2, 3, 0,
            4, 5, 6, 6, 7, 4,
            0, 1, 5, 5, 4, 0,
            2, 3, 7, 7, 6, 2,
            0, 3, 7, 7, 4, 0,
            1, 2, 6, 6, 5, 1,
        ];

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Cube Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Cube Index Buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let index_count = indices.len() as u32;

        Mesh {
            vertex_buffer,
            index_buffer,
            index_count,
        }
    }
}

#[derive(Debug, Component)]
pub struct MeshRenderer {
    pub mesh: AssetHandle<Mesh>,
    pub material: AssetHandle<Material>,
}
