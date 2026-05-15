use raylib::camera::Camera3D;
use raylib::color::Color;
use raylib::prelude::{RaylibDraw as _, RaylibDraw3D as _, RaylibMode3DExt as _};
use raylib::{RaylibHandle, RaylibThread};

use crate::ecs::{Camera, Projection, Transform};
use crate::engine::{Backend, Scene};
use crate::util::IntoVector3;

pub struct Renderer {}

impl Renderer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render_scene<B: Backend>(
        &mut self,
        rl: &mut RaylibHandle,
        rl_thread: &RaylibThread,
        scene: &mut Scene<B>,
    ) {
        todo!();
        // let Some(cam_entity) = scene.active_camera() else { return };

        // let transform = scene.query_in::<Transform>(cam_entity).next().unwrap();
        // let camera = scene.query_in::<Camera>(cam_entity).next().unwrap();

        // let pos = transform.translation().to_rl();
        // let forward = glam::Vec3::Z;
        // let target = (transform.rotation() * forward).to_rl() + pos;

        // let up = camera.up.to_rl();

        // let rl_camera = match camera.projection {
        //     Projection::Perspective { fov_y } => Camera3D::perspective(pos, target, up, fov_y),
        //     Projection::Orthographic { fov_y } => Camera3D::orthographic(pos, target, up, fov_y),
        // };

        // let mut d = rl.begin_drawing(&rl_thread);
        // d.clear_background(Color::RAYWHITE);
        // d.draw_mode3D(rl_camera, |mut d, _| {
        //     d.draw_grid(10, 10.0);
        // });
        // d.draw_fps(10, 10);
    }
}
