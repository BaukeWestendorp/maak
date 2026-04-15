use std::time::Instant;

use crate::engine::{Backend, KeyboardKey, MouseButton, Scene};
use crate::util::{IntoVector2 as _, ToGlamVector2 as _};

mod input;

mod renderer;

pub struct RaylibBackend {
    rl: raylib::RaylibHandle,
    thread: raylib::RaylibThread,
}

impl Default for RaylibBackend {
    fn default() -> Self {
        let (rl, thread) = raylib::init()
            .title("Rico Application")
            .size(1080, 720)
            .resizable()
            .vsync()
            .msaa_4x()
            .build();

        Self { rl, thread }
    }
}

impl Backend for RaylibBackend {
    fn run_scene(&mut self, scene: &mut Scene<Self>) {
        let mut renderer = renderer::Renderer::new();

        scene.setup(self);

        let mut last_frame = Instant::now();
        while !self.rl.window_should_close() {
            let now = Instant::now();
            let delta_time = (now - last_frame).as_secs_f32();
            last_frame = now;

            scene.update(delta_time, self);

            renderer.render_scene(&mut self.rl, &mut self.thread, scene);
        }

        scene.shutdown(self);
    }

    #[inline]
    fn is_key_pressed(&self, key: KeyboardKey) -> bool {
        self.rl.is_key_pressed(key.into())
    }

    #[inline]
    fn is_key_pressed_repeat(&self, key: KeyboardKey) -> bool {
        self.rl.is_key_pressed_repeat(key.into())
    }

    #[inline]
    fn is_key_down(&self, key: KeyboardKey) -> bool {
        self.rl.is_key_down(key.into())
    }

    #[inline]
    fn is_key_released(&self, key: KeyboardKey) -> bool {
        self.rl.is_key_released(key.into())
    }

    #[inline]
    fn is_key_up(&self, key: KeyboardKey) -> bool {
        self.rl.is_key_up(key.into())
    }

    #[inline]
    fn get_key_pressed(&mut self) -> Option<KeyboardKey> {
        self.rl.get_key_pressed().map(Into::into)
    }

    #[inline]
    fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        self.rl.is_mouse_button_pressed(button.into())
    }

    #[inline]
    fn is_mouse_button_down(&self, button: MouseButton) -> bool {
        self.rl.is_mouse_button_down(button.into())
    }

    #[inline]
    fn is_mouse_button_released(&self, button: MouseButton) -> bool {
        self.rl.is_mouse_button_released(button.into())
    }

    #[inline]
    fn is_mouse_button_up(&self, button: MouseButton) -> bool {
        self.rl.is_mouse_button_up(button.into())
    }

    #[inline]
    fn get_mouse_x(&self) -> i32 {
        self.rl.get_mouse_x()
    }

    #[inline]
    fn get_mouse_y(&self) -> i32 {
        self.rl.get_mouse_y()
    }

    #[inline]
    fn get_mouse_position(&self) -> glam::Vec2 {
        self.rl.get_mouse_position().to_glam()
    }

    #[inline]
    fn get_mouse_delta(&self) -> glam::Vec2 {
        self.rl.get_mouse_delta().to_glam()
    }

    #[inline]
    fn set_mouse_position(&mut self, position: glam::Vec2) {
        self.rl.set_mouse_position(position.to_rl())
    }

    #[inline]
    fn set_mouse_offset(&mut self, offset: glam::Vec2) {
        self.rl.set_mouse_offset(offset.to_rl())
    }

    #[inline]
    fn get_mouse_wheel_move(&self) -> f32 {
        self.rl.get_mouse_wheel_move()
    }

    #[inline]
    fn get_mouse_wheel_move_v(&self) -> glam::Vec2 {
        self.rl.get_mouse_wheel_move_v().to_glam()
    }
}
