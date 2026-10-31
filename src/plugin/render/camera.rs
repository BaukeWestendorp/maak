use bevy_ecs::resource::Resource;

use crate::{plugin::render::Uniform, runner::WgpuHandles};

#[derive(Debug, Resource)]
pub struct Camera {
    pub translation: glam::Vec3,
    pub rotation: glam::Quat,
    pub fov: f32,
    pub aspect_ratio: f32,
    pub near: f32,
    pub far: f32,
}

impl Camera {
    pub fn new(aspect_ratio: f32) -> Self {
        Self {
            translation: glam::Vec3::new(0.0, 0.0, 5.0),
            rotation: glam::Quat::IDENTITY,
            fov: 90.0_f32.to_radians(),
            aspect_ratio,
            near: 0.001,
            far: 1000.0,
        }
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
}

#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub(super) struct CameraUniformData {
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
pub(super) struct CameraBinding {
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
