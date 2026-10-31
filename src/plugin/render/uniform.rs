use wgpu::util::DeviceExt as _;

use crate::runner::WgpuHandles;

pub struct Uniform<T: bytemuck::Pod + bytemuck::Zeroable> {
    buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    _marker: std::marker::PhantomData<T>,
}

impl<T: bytemuck::Pod + bytemuck::Zeroable> Uniform<T> {
    pub fn new(
        WgpuHandles { device, .. }: &WgpuHandles,
        initial_value: &T,
        layout: &wgpu::BindGroupLayout,
        label: &str,
    ) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{label} Buffer")),
            contents: bytemuck::bytes_of(initial_value),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(&format!("{label} Bind Group")),
            layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });

        Self {
            buffer,
            bind_group,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn write(&self, wgpu_handles: &WgpuHandles, value: &T) {
        wgpu_handles
            .queue
            .write_buffer(&self.buffer, 0, bytemuck::bytes_of(value));
    }

    pub fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }
}
