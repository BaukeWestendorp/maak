use bevy_ecs::{
    resource::Resource,
    system::{Res, ResMut},
};

use crate::{
    plugin::render::Uniform,
    runner::{WgpuHandles, WindowHandle},
};

#[derive(Resource)]
pub struct Camera {
    pub translation: glam::Vec3,
    pub rotation: glam::Quat,
    pub fov: f32,
    pub aspect_ratio: f32,
    pub near: f32,
    pub far: f32,

    bind_group_layout: wgpu::BindGroupLayout,
    binding: CameraBinding,
}

impl Camera {
    pub fn new(aspect_ratio: f32, wgpu_handles: &WgpuHandles) -> Self {
        let bind_group_layout =
            wgpu_handles
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("Camera Bind Group Layout"),
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                });

        let binding = CameraBinding::new(wgpu_handles, &bind_group_layout);

        Self {
            translation: glam::Vec3::new(0.0, 0.0, 5.0),
            rotation: glam::Quat::IDENTITY,
            fov: 90.0_f32,
            aspect_ratio,
            near: 0.001,
            far: 1000.0,

            bind_group_layout,
            binding,
        }
    }

    pub fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }

    pub fn binding(&self) -> &CameraBinding {
        &self.binding
    }

    pub fn view_matrix(&self) -> glam::Mat4 {
        glam::Mat4::from_rotation_translation(self.rotation, self.translation).inverse()
    }

    pub fn proj_matrix(&self) -> glam::Mat4 {
        glam::camera::rh::proj::directx::perspective(
            self.fov.to_radians(),
            self.aspect_ratio,
            self.near,
            self.far,
        )
    }

    pub fn view_proj_matrix(&self) -> glam::Mat4 {
        self.proj_matrix() * self.view_matrix()
    }

    pub fn forward(&self) -> glam::Vec3 {
        self.rotation * glam::Vec3::NEG_Z
    }

    pub fn right(&self) -> glam::Vec3 {
        self.rotation * glam::Vec3::X
    }

    pub fn up(&self) -> glam::Vec3 {
        self.rotation * glam::Vec3::Y
    }
}

pub(super) fn update(
    window_handle: Res<WindowHandle>,
    wgpu_handles: Res<WgpuHandles>,
    mut camera: ResMut<Camera>,
) {
    camera.aspect_ratio = window_handle.aspect_ratio();

    let camera_uniform_data = CameraUniformData::new(&camera);

    camera
        .binding
        .uniform
        .write(&wgpu_handles, &camera_uniform_data);
}

#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct CameraUniformData {
    pub view_proj: [[f32; 4]; 4],
}

impl CameraUniformData {
    pub fn new(camera: &Camera) -> Self {
        Self {
            view_proj: camera.view_proj_matrix().to_cols_array_2d(),
        }
    }
}

#[derive(Resource)]
pub struct CameraBinding {
    pub uniform: Uniform<CameraUniformData>,
}

impl CameraBinding {
    pub fn new(wgpu_handles: &WgpuHandles, bind_group_layout: &wgpu::BindGroupLayout) -> Self {
        let data = CameraUniformData {
            view_proj: glam::Mat4::IDENTITY.to_cols_array_2d(),
        };

        Self {
            uniform: Uniform::new(wgpu_handles, &data, bind_group_layout, "Camera Uniform"),
        }
    }
}
