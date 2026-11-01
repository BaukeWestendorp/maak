use bevy_ecs::{
    resource::Resource,
    system::{Res, ResMut},
};

use crate::{
    engine::Engine,
    input::{InputState, KeyboardKey, MouseButton},
    plugin::{Plugin, render::Camera, time::Time},
    runner::Runner,
};

pub struct CameraControlPlugin;

impl<R: Runner> Plugin<R> for CameraControlPlugin {
    fn build(&self, engine: &mut Engine<R>) {
        engine.world().insert_resource(CameraController::default());
        engine.add_update_system(update);
    }
}

#[derive(Resource)]
pub struct CameraController {
    pub speed: f32,
    pub mouse_sensitivity: f32,

    pub pitch: f32,
    pub yaw: f32,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            speed: 5.0,
            mouse_sensitivity: 0.3,

            pitch: 0.0,
            yaw: 0.0,
        }
    }
}

fn update(
    mut controller: ResMut<CameraController>,
    mut camera: ResMut<Camera>,
    mut input_state: ResMut<InputState>,
    time: Res<Time>,
) {
    let speed = controller.speed;
    let delta_sec = time.delta_seconds();
    let mouse_delta = input_state.mouse.cursor.delta;
    let is_dragging = input_state.mouse.is_button_down(MouseButton::Right);

    input_state.mouse.cursor.visible = !is_dragging;
    input_state.mouse.cursor.locked = is_dragging;

    if mouse_delta != glam::Vec2::ZERO && is_dragging {
        controller.yaw -= mouse_delta.x * controller.mouse_sensitivity * delta_sec;
        controller.pitch -= mouse_delta.y * controller.mouse_sensitivity * delta_sec;

        controller.pitch = controller
            .pitch
            .clamp(-89.0_f32.to_radians(), 89.0_f32.to_radians());

        camera.rotation =
            glam::Quat::from_euler(glam::EulerRot::YXZ, controller.yaw, controller.pitch, 0.0);
    }

    let mut movement = glam::Vec3::ZERO;

    let mut forward = camera.forward();
    forward.y = 0.0;
    forward = forward.normalize_or_zero();

    let mut right = camera.right();
    right.y = 0.0;
    right = right.normalize_or_zero();

    if input_state.keyboard.is_key_down(KeyboardKey::W) {
        movement += forward;
    }
    if input_state.keyboard.is_key_down(KeyboardKey::S) {
        movement -= forward;
    }
    if input_state.keyboard.is_key_down(KeyboardKey::D) {
        movement += right;
    }
    if input_state.keyboard.is_key_down(KeyboardKey::A) {
        movement -= right;
    }
    if input_state.keyboard.is_key_down(KeyboardKey::Space) {
        movement += glam::Vec3::Y;
    }
    if input_state.keyboard.is_key_down(KeyboardKey::LeftShift) {
        movement -= glam::Vec3::Y;
    }

    if movement != glam::Vec3::ZERO {
        camera.translation += movement.normalize_or_zero() * speed * delta_sec;
    }
}
