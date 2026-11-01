use std::collections::HashSet;

use bevy_ecs::resource::Resource;

#[derive(Debug, Clone, PartialEq, Default, Resource)]
pub struct InputState {
    pub keyboard: KeyboardState,
    pub mouse: MouseState,
}

impl InputState {
    pub fn clear_frame_state(&mut self) {
        self.keyboard.clear_frame_state();
        self.mouse.clear_frame_state();
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct KeyboardState {
    pub held_keys: HashSet<KeyboardKey>,
    pub just_pressed_keys: HashSet<KeyboardKey>,
    pub just_released_keys: HashSet<KeyboardKey>,
}

impl KeyboardState {
    pub fn is_key_down(&self, key: KeyboardKey) -> bool {
        self.held_keys.contains(&key)
    }

    pub fn is_key_just_pressed(&self, key: KeyboardKey) -> bool {
        self.just_pressed_keys.contains(&key)
    }

    pub fn is_key_just_released(&self, key: KeyboardKey) -> bool {
        self.just_released_keys.contains(&key)
    }

    pub fn is_shift_down(&self) -> bool {
        self.is_key_down(KeyboardKey::LeftShift) || self.is_key_down(KeyboardKey::RightShift)
    }

    pub fn is_ctrl_down(&self) -> bool {
        self.is_key_down(KeyboardKey::LeftControl) || self.is_key_down(KeyboardKey::RightControl)
    }

    pub fn is_alt_down(&self) -> bool {
        self.is_key_down(KeyboardKey::LeftAlt) || self.is_key_down(KeyboardKey::RightAlt)
    }

    pub fn clear_frame_state(&mut self) {
        self.just_pressed_keys.clear();
        self.just_released_keys.clear();
    }
}

#[rustfmt::skip]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyboardKey {
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    One, Two, Three, Four, Five, Six, Seven, Eight, Nine, Zero,

    Tab, CapsLock, Control, Alt, Space, Enter, Backspace, Escape,
    LeftShift, RightShift,
    LeftControl, RightControl,
    LeftAlt, RightAlt,
    LeftSuper, RightSuper,

    LeftArrow, RightArrow, UpArrow, DownArrow,
    Insert, Delete, Home, End, PageUp, PageDown,

    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
    F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24,

    Grave,
    Minus,
    Equal,
    LeftBracket,
    RightBracket,
    Backslash,
    Semicolon,
    Apostrophe,
    Comma,
    Period,
    Slash,

    NumLock,
    Numpad0, Numpad1, Numpad2, Numpad3, Numpad4,
    Numpad5, Numpad6, Numpad7, Numpad8, Numpad9,
    NumpadAdd, NumpadSubtract, NumpadMultiply, NumpadDivide,
    NumpadDecimal, NumpadEnter, NumpadEqual,

    PrintScreen, ScrollLock, Pause,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct MouseState {
    pub cursor: CursorState,
    pub scroll_delta: glam::Vec2,
    pub held_buttons: HashSet<MouseButton>,
    pub just_pressed_buttons: HashSet<MouseButton>,
    pub just_released_buttons: HashSet<MouseButton>,
}

impl MouseState {
    pub fn is_button_down(&self, button: MouseButton) -> bool {
        self.held_buttons.contains(&button)
    }

    pub fn is_button_just_pressed(&self, button: MouseButton) -> bool {
        self.just_pressed_buttons.contains(&button)
    }

    pub fn is_button_just_released(&self, button: MouseButton) -> bool {
        self.just_released_buttons.contains(&button)
    }

    pub fn clear_frame_state(&mut self) {
        self.just_pressed_buttons.clear();
        self.just_released_buttons.clear();
        self.cursor.delta = glam::Vec2::ZERO;
        self.scroll_delta = glam::Vec2::ZERO;
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CursorState {
    pub position: glam::Vec2,
    pub delta: glam::Vec2,
    pub visible: bool,
    pub locked: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Forward,
    Back,
    Other(u16),
}
