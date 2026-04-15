mod rl;

pub use rl::*;

use crate::engine::{KeyboardKey, MouseButton, Scene};

pub trait Backend {
    fn run_scene(&mut self, scene: &mut Scene<Self>)
    where
        Self: Sized;

    /// Detect if a key has been pressed once.
    fn is_key_pressed(&self, key: KeyboardKey) -> bool;

    /// Check if a key has been pressed again
    fn is_key_pressed_repeat(&self, key: KeyboardKey) -> bool;

    /// Detect if a key is being pressed.
    fn is_key_down(&self, key: KeyboardKey) -> bool;

    /// Detect if a key has been released once.
    fn is_key_released(&self, key: KeyboardKey) -> bool;

    /// Detect if a key is NOT being pressed.
    fn is_key_up(&self, key: KeyboardKey) -> bool;

    /// Gets latest key pressed.
    fn get_key_pressed(&mut self) -> Option<KeyboardKey>;

    /// Detect if a mouse button has been pressed once.
    fn is_mouse_button_pressed(&self, button: MouseButton) -> bool;

    /// Detect if a mouse button is being pressed.
    fn is_mouse_button_down(&self, button: MouseButton) -> bool;

    /// Detect if a mouse button has been released once.
    fn is_mouse_button_released(&self, button: MouseButton) -> bool;

    /// Detect if a mouse button is NOT being pressed.
    fn is_mouse_button_up(&self, button: MouseButton) -> bool;

    /// Returns mouse position X.
    fn get_mouse_x(&self) -> i32;

    /// Returns mouse position Y.
    fn get_mouse_y(&self) -> i32;

    /// Returns mouse position.
    fn get_mouse_position(&self) -> glam::Vec2;

    /// Returns mouse delta between frames.
    fn get_mouse_delta(&self) -> glam::Vec2;

    /// Sets mouse position.
    fn set_mouse_position(&mut self, position: glam::Vec2);

    /// Sets mouse offset.
    fn set_mouse_offset(&mut self, offset: glam::Vec2);

    /// Get mouse wheel movement for X or Y, whichever is larger
    fn get_mouse_wheel_move(&self) -> f32;

    /// Get mouse wheel movement for both X and Y
    fn get_mouse_wheel_move_v(&self) -> glam::Vec2;
}
