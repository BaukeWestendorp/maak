use crate::prelude::*;

macro_rules! keyboard_key_from {
    ($($variant:ident => $raylib_key:ident),* $(,)?) => {
        impl From<KeyboardKey> for raylib::consts::KeyboardKey {
            fn from(value: KeyboardKey) -> Self {
                match value {
                    $(KeyboardKey::$variant => raylib::consts::KeyboardKey::$raylib_key,)*
                }
            }
        }

        impl From<raylib::consts::KeyboardKey> for KeyboardKey {
            fn from(value: raylib::consts::KeyboardKey) -> Self {
                match value {
                    $(raylib::consts::KeyboardKey::$raylib_key => KeyboardKey::$variant,)*
                    raylib::consts::KeyboardKey::KEY_NULL => todo!(),
                }
            }
        }
    };
}

keyboard_key_from! {
    Apostrophe => KEY_APOSTROPHE,
    Comma => KEY_COMMA,
    Minus => KEY_MINUS,
    Period => KEY_PERIOD,
    Slash => KEY_SLASH,
    Zero => KEY_ZERO,
    One => KEY_ONE,
    Two => KEY_TWO,
    Three => KEY_THREE,
    Four => KEY_FOUR,
    Five => KEY_FIVE,
    Six => KEY_SIX,
    Seven => KEY_SEVEN,
    Eight => KEY_EIGHT,
    Nine => KEY_NINE,
    Semicolon => KEY_SEMICOLON,
    Equal => KEY_EQUAL,
    A => KEY_A,
    B => KEY_B,
    C => KEY_C,
    D => KEY_D,
    E => KEY_E,
    F => KEY_F,
    G => KEY_G,
    H => KEY_H,
    I => KEY_I,
    J => KEY_J,
    K => KEY_K,
    L => KEY_L,
    M => KEY_M,
    N => KEY_N,
    O => KEY_O,
    P => KEY_P,
    Q => KEY_Q,
    R => KEY_R,
    S => KEY_S,
    T => KEY_T,
    U => KEY_U,
    V => KEY_V,
    W => KEY_W,
    X => KEY_X,
    Y => KEY_Y,
    Z => KEY_Z,
    LeftBracket => KEY_LEFT_BRACKET,
    Backslash => KEY_BACKSLASH,
    RightBracket => KEY_RIGHT_BRACKET,
    Grave => KEY_GRAVE,
    Space => KEY_SPACE,
    Escape => KEY_ESCAPE,
    Enter => KEY_ENTER,
    Tab => KEY_TAB,
    Backspace => KEY_BACKSPACE,
    Insert => KEY_INSERT,
    Delete => KEY_DELETE,
    Right => KEY_RIGHT,
    Left => KEY_LEFT,
    Down => KEY_DOWN,
    Up => KEY_UP,
    PageUp => KEY_PAGE_UP,
    PageDown => KEY_PAGE_DOWN,
    Home => KEY_HOME,
    End => KEY_END,
    CapsLock => KEY_CAPS_LOCK,
    ScrollLock => KEY_SCROLL_LOCK,
    NumLock => KEY_NUM_LOCK,
    PrintScreen => KEY_PRINT_SCREEN,
    Pause => KEY_PAUSE,
    F1 => KEY_F1,
    F2 => KEY_F2,
    F3 => KEY_F3,
    F4 => KEY_F4,
    F5 => KEY_F5,
    F6 => KEY_F6,
    F7 => KEY_F7,
    F8 => KEY_F8,
    F9 => KEY_F9,
    F10 => KEY_F10,
    F11 => KEY_F11,
    F12 => KEY_F12,
    LeftShift => KEY_LEFT_SHIFT,
    LeftControl => KEY_LEFT_CONTROL,
    LeftAlt => KEY_LEFT_ALT,
    LeftSuper => KEY_LEFT_SUPER,
    RightShift => KEY_RIGHT_SHIFT,
    RightControl => KEY_RIGHT_CONTROL,
    RightAlt => KEY_RIGHT_ALT,
    RightSuper => KEY_RIGHT_SUPER,
    KbMenu => KEY_KB_MENU,
    Kp0 => KEY_KP_0,
    Kp1 => KEY_KP_1,
    Kp2 => KEY_KP_2,
    Kp3 => KEY_KP_3,
    Kp4 => KEY_KP_4,
    Kp5 => KEY_KP_5,
    Kp6 => KEY_KP_6,
    Kp7 => KEY_KP_7,
    Kp8 => KEY_KP_8,
    Kp9 => KEY_KP_9,
    KpDecimal => KEY_KP_DECIMAL,
    KpDivide => KEY_KP_DIVIDE,
    KpMultiply => KEY_KP_MULTIPLY,
    KpSubtract => KEY_KP_SUBTRACT,
    KpAdd => KEY_KP_ADD,
    KpEnter => KEY_KP_ENTER,
    KpEqual => KEY_KP_EQUAL,
    Back => KEY_BACK,
    Menu => KEY_MENU,
    VolumeUp => KEY_VOLUME_UP,
    VolumeDown => KEY_VOLUME_DOWN,
}

macro_rules! mouse_button_from {
    ($($variant:ident => $raylib_button:ident),* $(,)?) => {
        impl From<MouseButton> for raylib::consts::MouseButton {
            fn from(value: MouseButton) -> Self {
                match value {
                    $(MouseButton::$variant => raylib::consts::MouseButton::$raylib_button,)*
                }
            }
        }

        impl From<raylib::consts::MouseButton> for MouseButton {
            fn from(value: raylib::consts::MouseButton) -> Self {
                match value {
                    $(raylib::consts::MouseButton::$raylib_button => MouseButton::$variant,)*
                }
            }
        }
    };
}

mouse_button_from! {
    Left => MOUSE_BUTTON_LEFT,
    Right => MOUSE_BUTTON_RIGHT,
    Middle => MOUSE_BUTTON_MIDDLE,
    Side => MOUSE_BUTTON_SIDE,
    Extra => MOUSE_BUTTON_EXTRA,
    Forward => MOUSE_BUTTON_FORWARD,
    Back => MOUSE_BUTTON_BACK,
}

macro_rules! mouse_cursor_from {
    ($($variant:ident => $raylib_cursor:ident),* $(,)?) => {
        impl From<MouseCursor> for raylib::consts::MouseCursor {
            fn from(value: MouseCursor) -> Self {
                match value {
                    $(MouseCursor::$variant => raylib::consts::MouseCursor::$raylib_cursor,)*
                }
            }
        }

        impl From<raylib::consts::MouseCursor> for MouseCursor {
            fn from(value: raylib::consts::MouseCursor) -> Self {
                match value {
                    $(raylib::consts::MouseCursor::$raylib_cursor => MouseCursor::$variant,)*
                }
            }
        }
    };
}

mouse_cursor_from! {
    Default => MOUSE_CURSOR_DEFAULT,
    Arrow => MOUSE_CURSOR_ARROW,
    Ibeam => MOUSE_CURSOR_IBEAM,
    Crosshair => MOUSE_CURSOR_CROSSHAIR,
    PointingHand => MOUSE_CURSOR_POINTING_HAND,
    ResizeEw => MOUSE_CURSOR_RESIZE_EW,
    ResizeNs => MOUSE_CURSOR_RESIZE_NS,
    ResizeNwse => MOUSE_CURSOR_RESIZE_NWSE,
    ResizeNesw => MOUSE_CURSOR_RESIZE_NESW,
    ResizeAll => MOUSE_CURSOR_RESIZE_ALL,
    NotAllowed => MOUSE_CURSOR_NOT_ALLOWED,
}

macro_rules! gamepad_button_from {
    ($($variant:ident => $raylib_button:ident),* $(,)?) => {
        impl From<GamepadButton> for raylib::consts::GamepadButton {
            fn from(value: GamepadButton) -> Self {
                match value {
                    $(GamepadButton::$variant => raylib::consts::GamepadButton::$raylib_button,)*
                }
            }
        }

        impl From<raylib::consts::GamepadButton> for GamepadButton {
            fn from(value: raylib::consts::GamepadButton) -> Self {
                match value {
                    $(raylib::consts::GamepadButton::$raylib_button => GamepadButton::$variant,)*
                }
            }
        }
    };
}

gamepad_button_from! {
    Unknown => GAMEPAD_BUTTON_UNKNOWN,
    LeftFaceUp => GAMEPAD_BUTTON_LEFT_FACE_UP,
    LeftFaceRight => GAMEPAD_BUTTON_LEFT_FACE_RIGHT,
    LeftFaceDown => GAMEPAD_BUTTON_LEFT_FACE_DOWN,
    LeftFaceLeft => GAMEPAD_BUTTON_LEFT_FACE_LEFT,
    RightFaceUp => GAMEPAD_BUTTON_RIGHT_FACE_UP,
    RightFaceRight => GAMEPAD_BUTTON_RIGHT_FACE_RIGHT,
    RightFaceDown => GAMEPAD_BUTTON_RIGHT_FACE_DOWN,
    RightFaceLeft => GAMEPAD_BUTTON_RIGHT_FACE_LEFT,
    LeftTrigger1 => GAMEPAD_BUTTON_LEFT_TRIGGER_1,
    LeftTrigger2 => GAMEPAD_BUTTON_LEFT_TRIGGER_2,
    RightTrigger1 => GAMEPAD_BUTTON_RIGHT_TRIGGER_1,
    RightTrigger2 => GAMEPAD_BUTTON_RIGHT_TRIGGER_2,
    MiddleLeft => GAMEPAD_BUTTON_MIDDLE_LEFT,
    Middle => GAMEPAD_BUTTON_MIDDLE,
    MiddleRight => GAMEPAD_BUTTON_MIDDLE_RIGHT,
    LeftThumb => GAMEPAD_BUTTON_LEFT_THUMB,
    RightThumb => GAMEPAD_BUTTON_RIGHT_THUMB,
}

macro_rules! gamepad_axis_from {
    ($($variant:ident => $raylib_axis:ident),* $(,)?) => {
        impl From<GamepadAxis> for raylib::consts::GamepadAxis {
            fn from(value: GamepadAxis) -> Self {
                match value {
                    $(GamepadAxis::$variant => raylib::consts::GamepadAxis::$raylib_axis,)*
                }
            }
        }

        impl From<raylib::consts::GamepadAxis> for GamepadAxis {
            fn from(value: raylib::consts::GamepadAxis) -> Self {
                match value {
                    $(raylib::consts::GamepadAxis::$raylib_axis => GamepadAxis::$variant,)*
                }
            }
        }
    };
}

gamepad_axis_from! {
    LeftX => GAMEPAD_AXIS_LEFT_X,
    LeftY => GAMEPAD_AXIS_LEFT_Y,
    RightX => GAMEPAD_AXIS_RIGHT_X,
    RightY => GAMEPAD_AXIS_RIGHT_Y,
    LeftTrigger => GAMEPAD_AXIS_LEFT_TRIGGER,
    RightTrigger => GAMEPAD_AXIS_RIGHT_TRIGGER,
}
