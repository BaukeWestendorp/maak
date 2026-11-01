use std::{ops::Deref, sync::Arc};

use crate::{
    engine::Engine,
    input::{InputState, KeyboardKey},
    runner::{RenderTarget, Runner, WgpuHandles, WindowHandle},
};

pub struct StandardRunner;

impl Runner for StandardRunner {
    fn run(self, engine: &mut Engine<Self>) {
        let mut app = App::new(engine);
        let event_loop = winit::event_loop::EventLoop::new().unwrap();
        event_loop.run_app(&mut app).unwrap();
    }
}

struct App<'engine> {
    started: bool,
    engine: &'engine mut Engine<StandardRunner>,
    window: Option<Arc<winit::window::Window>>,
}

impl<'engine> App<'engine> {
    fn new(engine: &'engine mut Engine<StandardRunner>) -> Self {
        Self {
            started: false,
            engine,
            window: None,
        }
    }

    fn start(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.started = true;
        let window_handle = window_handle(event_loop);

        let wgpu_handles =
            pollster::block_on(async { wgpu_handles(Arc::clone(&window_handle.deref())).await })
                .expect("Failed to initialize wgpu");

        self.window = Some(Arc::clone(&window_handle.deref()));

        self.engine.world().insert_resource(window_handle);
        self.engine.world().insert_resource(wgpu_handles);
        self.engine.world().insert_resource(InputState::default());

        self.engine.build_plugins();

        self.engine.setup();
    }
}

impl winit::application::ApplicationHandler for App<'_> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if !self.started {
            self.start(event_loop);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            winit::event::WindowEvent::KeyboardInput { event, .. } => {
                let Some(mut input_state) = self.engine.world().get_resource_mut::<InputState>()
                else {
                    return;
                };

                if let winit::keyboard::PhysicalKey::Code(key_code) = event.physical_key {
                    if let Ok(key) = KeyboardKey::try_from(key_code) {
                        if event.state.is_pressed() {
                            if !input_state.keyboard.held_keys.contains(&key) {
                                input_state.keyboard.just_pressed_keys.insert(key);
                                input_state.keyboard.held_keys.insert(key);
                            }
                        } else {
                            input_state.keyboard.held_keys.remove(&key);
                            input_state.keyboard.just_released_keys.insert(key);
                        }
                    }
                }
            }
            winit::event::WindowEvent::CursorMoved { position, .. } => {
                let Some(mut input_state) = self.engine.world().get_resource_mut::<InputState>()
                else {
                    return;
                };

                input_state.mouse.cursor.position =
                    glam::vec2(position.x as f32, position.y as f32);
            }
            winit::event::WindowEvent::Resized(size) => {
                if size.width == 0 || size.height == 0 {
                    return;
                }

                let Some(wgpu_handles) = self.engine.world().get_resource_mut::<WgpuHandles>()
                else {
                    return;
                };

                let RenderTarget::Surface(surface) = &wgpu_handles.render_target else {
                    return;
                };

                surface.configure(
                    &wgpu_handles.device,
                    &wgpu::SurfaceConfiguration {
                        present_mode: wgpu::PresentMode::AutoVsync,
                        ..surface
                            .get_default_config(&wgpu_handles.adapter, size.width, size.height)
                            .expect("Failed to get default surface configuration")
                    },
                );
            }
            winit::event::WindowEvent::RedrawRequested => {
                self.engine.update();

                if let Some(mut input_state) = self.engine.world().get_resource_mut::<InputState>()
                {
                    input_state.clear_frame_state();

                    if let Some(window) = &self.window {
                        window.set_cursor_visible(input_state.mouse.cursor.visible);
                        let _ = window.set_cursor_grab(if input_state.mouse.cursor.locked {
                            winit::window::CursorGrabMode::Locked
                        } else {
                            winit::window::CursorGrabMode::None
                        });
                    }
                }

                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            winit::event::WindowEvent::CloseRequested | winit::event::WindowEvent::Destroyed => {
                event_loop.exit();
            }
            _ => {}
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        match event {
            winit::event::DeviceEvent::MouseMotion { delta } => {
                let Some(mut input_state) = self.engine.world().get_resource_mut::<InputState>()
                else {
                    return;
                };

                input_state.mouse.cursor.delta += glam::vec2(delta.0 as f32, delta.1 as f32);
            }
            winit::event::DeviceEvent::MouseWheel { delta } => {
                let Some(mut input_state) = self.engine.world().get_resource_mut::<InputState>()
                else {
                    return;
                };
                match delta {
                    winit::event::MouseScrollDelta::LineDelta(x, y) => {
                        input_state.mouse.scroll_delta += glam::vec2(x, y);
                    }
                    winit::event::MouseScrollDelta::PixelDelta(pos) => {
                        input_state.mouse.scroll_delta +=
                            glam::vec2(pos.x as f32, pos.y as f32) * 0.05;
                    }
                }
            }
            winit::event::DeviceEvent::Button { button, state } => {
                let Some(mut input_state) = self.engine.world().get_resource_mut::<InputState>()
                else {
                    return;
                };

                let mouse_button = match button {
                    0 => crate::input::MouseButton::Left,
                    1 => crate::input::MouseButton::Right,
                    2 => crate::input::MouseButton::Middle,
                    3 => crate::input::MouseButton::Forward,
                    4 => crate::input::MouseButton::Back,
                    other => crate::input::MouseButton::Other(other as u16),
                };

                if state.is_pressed() {
                    if !input_state.mouse.held_buttons.contains(&mouse_button) {
                        input_state.mouse.just_pressed_buttons.insert(mouse_button);
                        input_state.mouse.held_buttons.insert(mouse_button);
                    }
                } else {
                    input_state.mouse.held_buttons.remove(&mouse_button);
                    input_state.mouse.just_released_buttons.insert(mouse_button);
                }
            }
            _ => {}
        }
    }
}

fn window_handle(event_loop: &winit::event_loop::ActiveEventLoop) -> WindowHandle {
    let window = Arc::new(
        event_loop
            .create_window(winit::window::WindowAttributes::default())
            .unwrap(),
    );

    WindowHandle(Arc::clone(&window))
}

async fn wgpu_handles(window: Arc<winit::window::Window>) -> anyhow::Result<WgpuHandles> {
    let width = window.inner_size().width;
    let height = window.inner_size().height;

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::PRIMARY,
        flags: Default::default(),
        memory_budget_thresholds: Default::default(),
        backend_options: Default::default(),
        display: None,
    });

    let surface = instance.create_surface(window)?;

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
            apply_limit_buckets: true,
        })
        .await?;

    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::empty(),
            experimental_features: wgpu::ExperimentalFeatures::disabled(),
            required_limits: wgpu::Limits::default(),
            memory_hints: wgpu::MemoryHints::default(),
            trace: wgpu::Trace::Off,
        })
        .await?;

    surface.configure(
        &device,
        &wgpu::SurfaceConfiguration {
            present_mode: wgpu::PresentMode::AutoVsync,
            ..surface
                .get_default_config(&adapter, width, height)
                .expect("Failed to get default surface configuration")
        },
    );

    let target_format = surface
        .get_capabilities(&adapter)
        .formats
        .first()
        .cloned()
        .expect("Failed to get wgpu surface format");

    Ok(WgpuHandles {
        render_target: RenderTarget::Surface(surface),
        target_format,
        adapter,
        device,
        queue,
    })
}

impl TryFrom<winit::keyboard::KeyCode> for KeyboardKey {
    type Error = ();

    fn try_from(key: winit::keyboard::KeyCode) -> Result<Self, Self::Error> {
        let mapped = match key {
            winit::keyboard::KeyCode::KeyA => KeyboardKey::A,
            winit::keyboard::KeyCode::KeyB => KeyboardKey::B,
            winit::keyboard::KeyCode::KeyC => KeyboardKey::C,
            winit::keyboard::KeyCode::KeyD => KeyboardKey::D,
            winit::keyboard::KeyCode::KeyE => KeyboardKey::E,
            winit::keyboard::KeyCode::KeyF => KeyboardKey::F,
            winit::keyboard::KeyCode::KeyG => KeyboardKey::G,
            winit::keyboard::KeyCode::KeyH => KeyboardKey::H,
            winit::keyboard::KeyCode::KeyI => KeyboardKey::I,
            winit::keyboard::KeyCode::KeyJ => KeyboardKey::J,
            winit::keyboard::KeyCode::KeyK => KeyboardKey::K,
            winit::keyboard::KeyCode::KeyL => KeyboardKey::L,
            winit::keyboard::KeyCode::KeyM => KeyboardKey::M,
            winit::keyboard::KeyCode::KeyN => KeyboardKey::N,
            winit::keyboard::KeyCode::KeyO => KeyboardKey::O,
            winit::keyboard::KeyCode::KeyP => KeyboardKey::P,
            winit::keyboard::KeyCode::KeyQ => KeyboardKey::Q,
            winit::keyboard::KeyCode::KeyR => KeyboardKey::R,
            winit::keyboard::KeyCode::KeyS => KeyboardKey::S,
            winit::keyboard::KeyCode::KeyT => KeyboardKey::T,
            winit::keyboard::KeyCode::KeyU => KeyboardKey::U,
            winit::keyboard::KeyCode::KeyV => KeyboardKey::V,
            winit::keyboard::KeyCode::KeyW => KeyboardKey::W,
            winit::keyboard::KeyCode::KeyX => KeyboardKey::X,
            winit::keyboard::KeyCode::KeyY => KeyboardKey::Y,
            winit::keyboard::KeyCode::KeyZ => KeyboardKey::Z,

            winit::keyboard::KeyCode::Digit0 => KeyboardKey::Zero,
            winit::keyboard::KeyCode::Digit1 => KeyboardKey::One,
            winit::keyboard::KeyCode::Digit2 => KeyboardKey::Two,
            winit::keyboard::KeyCode::Digit3 => KeyboardKey::Three,
            winit::keyboard::KeyCode::Digit4 => KeyboardKey::Four,
            winit::keyboard::KeyCode::Digit5 => KeyboardKey::Five,
            winit::keyboard::KeyCode::Digit6 => KeyboardKey::Six,
            winit::keyboard::KeyCode::Digit7 => KeyboardKey::Seven,
            winit::keyboard::KeyCode::Digit8 => KeyboardKey::Eight,
            winit::keyboard::KeyCode::Digit9 => KeyboardKey::Nine,

            winit::keyboard::KeyCode::ArrowDown => KeyboardKey::DownArrow,
            winit::keyboard::KeyCode::ArrowLeft => KeyboardKey::LeftArrow,
            winit::keyboard::KeyCode::ArrowRight => KeyboardKey::RightArrow,
            winit::keyboard::KeyCode::ArrowUp => KeyboardKey::UpArrow,
            winit::keyboard::KeyCode::Escape => KeyboardKey::Escape,
            winit::keyboard::KeyCode::Tab => KeyboardKey::Tab,
            winit::keyboard::KeyCode::Backspace => KeyboardKey::Backspace,
            winit::keyboard::KeyCode::Enter => KeyboardKey::Enter,
            winit::keyboard::KeyCode::Space => KeyboardKey::Space,
            winit::keyboard::KeyCode::Insert => KeyboardKey::Insert,
            winit::keyboard::KeyCode::Delete => KeyboardKey::Delete,
            winit::keyboard::KeyCode::Home => KeyboardKey::Home,
            winit::keyboard::KeyCode::End => KeyboardKey::End,
            winit::keyboard::KeyCode::PageUp => KeyboardKey::PageUp,
            winit::keyboard::KeyCode::PageDown => KeyboardKey::PageDown,

            winit::keyboard::KeyCode::CapsLock => KeyboardKey::CapsLock,
            winit::keyboard::KeyCode::ShiftLeft => KeyboardKey::LeftShift,
            winit::keyboard::KeyCode::ShiftRight => KeyboardKey::RightShift,
            winit::keyboard::KeyCode::ControlLeft => KeyboardKey::LeftControl,
            winit::keyboard::KeyCode::ControlRight => KeyboardKey::RightControl,
            winit::keyboard::KeyCode::AltLeft => KeyboardKey::LeftAlt,
            winit::keyboard::KeyCode::AltRight => KeyboardKey::RightAlt,
            winit::keyboard::KeyCode::SuperLeft => KeyboardKey::LeftSuper,
            winit::keyboard::KeyCode::SuperRight => KeyboardKey::RightSuper,

            winit::keyboard::KeyCode::F1 => KeyboardKey::F1,
            winit::keyboard::KeyCode::F2 => KeyboardKey::F2,
            winit::keyboard::KeyCode::F3 => KeyboardKey::F3,
            winit::keyboard::KeyCode::F4 => KeyboardKey::F4,
            winit::keyboard::KeyCode::F5 => KeyboardKey::F5,
            winit::keyboard::KeyCode::F6 => KeyboardKey::F6,
            winit::keyboard::KeyCode::F7 => KeyboardKey::F7,
            winit::keyboard::KeyCode::F8 => KeyboardKey::F8,
            winit::keyboard::KeyCode::F9 => KeyboardKey::F9,
            winit::keyboard::KeyCode::F10 => KeyboardKey::F10,
            winit::keyboard::KeyCode::F11 => KeyboardKey::F11,
            winit::keyboard::KeyCode::F12 => KeyboardKey::F12,
            winit::keyboard::KeyCode::F13 => KeyboardKey::F13,
            winit::keyboard::KeyCode::F14 => KeyboardKey::F14,
            winit::keyboard::KeyCode::F15 => KeyboardKey::F15,
            winit::keyboard::KeyCode::F16 => KeyboardKey::F16,
            winit::keyboard::KeyCode::F17 => KeyboardKey::F17,
            winit::keyboard::KeyCode::F18 => KeyboardKey::F18,
            winit::keyboard::KeyCode::F19 => KeyboardKey::F19,
            winit::keyboard::KeyCode::F20 => KeyboardKey::F20,
            winit::keyboard::KeyCode::F21 => KeyboardKey::F21,
            winit::keyboard::KeyCode::F22 => KeyboardKey::F22,
            winit::keyboard::KeyCode::F23 => KeyboardKey::F23,
            winit::keyboard::KeyCode::F24 => KeyboardKey::F24,

            winit::keyboard::KeyCode::Backquote => KeyboardKey::Grave,
            winit::keyboard::KeyCode::Minus => KeyboardKey::Minus,
            winit::keyboard::KeyCode::Equal => KeyboardKey::Equal,
            winit::keyboard::KeyCode::BracketLeft => KeyboardKey::LeftBracket,
            winit::keyboard::KeyCode::BracketRight => KeyboardKey::RightBracket,
            winit::keyboard::KeyCode::Backslash => KeyboardKey::Backslash,
            winit::keyboard::KeyCode::Semicolon => KeyboardKey::Semicolon,
            winit::keyboard::KeyCode::Quote => KeyboardKey::Apostrophe,
            winit::keyboard::KeyCode::Comma => KeyboardKey::Comma,
            winit::keyboard::KeyCode::Period => KeyboardKey::Period,
            winit::keyboard::KeyCode::Slash => KeyboardKey::Slash,

            winit::keyboard::KeyCode::NumLock => KeyboardKey::NumLock,
            winit::keyboard::KeyCode::Numpad0 => KeyboardKey::Numpad0,
            winit::keyboard::KeyCode::Numpad1 => KeyboardKey::Numpad1,
            winit::keyboard::KeyCode::Numpad2 => KeyboardKey::Numpad2,
            winit::keyboard::KeyCode::Numpad3 => KeyboardKey::Numpad3,
            winit::keyboard::KeyCode::Numpad4 => KeyboardKey::Numpad4,
            winit::keyboard::KeyCode::Numpad5 => KeyboardKey::Numpad5,
            winit::keyboard::KeyCode::Numpad6 => KeyboardKey::Numpad6,
            winit::keyboard::KeyCode::Numpad7 => KeyboardKey::Numpad7,
            winit::keyboard::KeyCode::Numpad8 => KeyboardKey::Numpad8,
            winit::keyboard::KeyCode::Numpad9 => KeyboardKey::Numpad9,
            winit::keyboard::KeyCode::NumpadAdd => KeyboardKey::NumpadAdd,
            winit::keyboard::KeyCode::NumpadSubtract => KeyboardKey::NumpadSubtract,
            winit::keyboard::KeyCode::NumpadMultiply => KeyboardKey::NumpadMultiply,
            winit::keyboard::KeyCode::NumpadDivide => KeyboardKey::NumpadDivide,
            winit::keyboard::KeyCode::NumpadDecimal => KeyboardKey::NumpadDecimal,
            winit::keyboard::KeyCode::NumpadEnter => KeyboardKey::NumpadEnter,
            winit::keyboard::KeyCode::NumpadEqual => KeyboardKey::NumpadEqual,

            winit::keyboard::KeyCode::PrintScreen => KeyboardKey::PrintScreen,
            winit::keyboard::KeyCode::ScrollLock => KeyboardKey::ScrollLock,
            winit::keyboard::KeyCode::Pause => KeyboardKey::Pause,

            _ => return Err(()),
        };

        Ok(mapped)
    }
}
