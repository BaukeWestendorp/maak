use crate::ecs::Component;
use crate::engine::Backend;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Camera {
    pub projection: Projection,
    pub up: glam::Vec3,
}

impl Camera {
    pub fn perspective(fov_y: f32) -> Self {
        Self { projection: Projection::Perspective { fov_y }, up: glam::Vec3::Y }
    }

    pub fn orthographic(fov_y: f32) -> Self {
        Self { projection: Projection::Orthographic { fov_y }, up: glam::Vec3::Y }
    }

    pub fn new() -> Self {
        Self::perspective(90.0)
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}

impl<B: Backend> Component<B> for Camera {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Projection {
    Perspective { fov_y: f32 },
    Orthographic { fov_y: f32 },
}
