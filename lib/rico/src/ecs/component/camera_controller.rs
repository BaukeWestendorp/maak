use glam::Vec3;

use crate::ecs::{Component, Transform};
use crate::engine::{Backend, Scene};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CameraController {
    mode: CameraMode,
}

impl CameraController {
    pub fn new() -> Self {
        Self { mode: CameraMode::Orbit { center: Vec3::ZERO, distance: 4.0 } }
    }
}

impl Default for CameraController {
    fn default() -> Self {
        Self::new()
    }
}

impl<B: Backend> Component<B> for CameraController {
    fn update(&mut self, _delta_time: f32, entity: hecs::Entity, scene: &Scene<B>, cx: &mut B) {
        match &mut self.mode {
            CameraMode::Orbit { center, distance } => {
                todo!();
                // let transform = scene.query_in::<Transform>(entity).next().unwrap();

                // let delta = cx.get_mouse_delta();
                // camera.orbit_around(center, delta);

                // if cx.get_mouse_wheel_move() != 0.0 {
                //     camera.zoom_towards(center, cx.get_mouse_wheel_move());
                // }
                // camera.look_at(center);
            }
            CameraMode::Free => {}
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CameraMode {
    Orbit { center: Vec3, distance: f32 },
    Free,
}
