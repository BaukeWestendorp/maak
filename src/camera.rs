use crate::prelude::*;

pub struct Camera {
    pub projection: Projection,
    width: usize,
    height: usize,
}

impl Camera {
    pub fn new(width: usize, height: usize) -> Self {
        Self { projection: Projection::default(), width, height }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.width as f32 / self.height as f32
    }
}

impl Object<Camera> {
    pub fn to_camera_space(&self, world_point: Vec3) -> Vec3 {
        self.affine.inverse().transform_point3(world_point)
    }

    pub fn project_camera_space(&self, dir: Vec3) -> Option<Vec2> {
        let depth = dir.z;
        if depth < 0.0001 {
            return None;
        }

        match self.projection {
            Projection::Orthographic => {
                let x = dir.x / self.aspect_ratio();
                let y = dir.y;
                Some(Vec2 { x, y })
            }
            Projection::Perspective { fov } => {
                let scale = (fov.to_radians() / 2.0).tan() * depth;
                let x = (dir.x / self.aspect_ratio()) / scale;
                let y = dir.y / scale;
                Some(Vec2 { x, y })
            }
        }
    }

    pub fn project(&self, point: impl Into<Vec3>) -> Option<Vec2> {
        let dir = self.to_camera_space(point.into());
        self.project_camera_space(dir)
    }

    pub fn clip_line_camera_space(&self, mut v0: Vec3, mut v1: Vec3) -> Option<(Vec3, Vec3)> {
        let near = 0.0001;

        if v0.z < near && v1.z < near {
            return None;
        }

        if v0.z < near {
            let t = (near - v0.z) / (v1.z - v0.z);
            v0 = v0.lerp(v1, t);
        } else if v1.z < near {
            let t = (near - v1.z) / (v0.z - v1.z);
            v1 = v1.lerp(v0, t);
        }

        Some((v0, v1))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Projection {
    Orthographic,
    Perspective { fov: f32 },
}

impl Default for Projection {
    fn default() -> Self {
        Self::Perspective { fov: 90.0 }
    }
}
