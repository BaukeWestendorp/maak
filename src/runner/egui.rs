use std::sync::Arc;

use bevy_ecs::resource::Resource;

use crate::{
    engine::Engine,
    input::{InputState, KeyboardKey},
    runner::{RenderTarget, Runner, WgpuHandles, WindowHandle},
};

pub struct EguiRunner {
    winit_window: Arc<winit::window::Window>,
    wgpu_render_state: eframe::egui_wgpu::RenderState,
}

impl EguiRunner {
    pub fn new(
        winit_window: Arc<winit::window::Window>,
        wgpu_render_state: eframe::egui_wgpu::RenderState,
    ) -> Self {
        Self {
            winit_window,
            wgpu_render_state,
        }
    }
}

impl Runner for EguiRunner {
    fn run(self, engine: &mut Engine<Self>) {
        let window_handle = WindowHandle(self.winit_window);

        let width = window_handle.inner_size().width;
        let height = window_handle.inner_size().height;

        let egui_texture_target = EguiTextureTarget::new(&self.wgpu_render_state, width, height);

        let wgpu_handles = WgpuHandles {
            render_target: egui_texture_target.render_target(),
            target_format: self.wgpu_render_state.target_format,
            adapter: self.wgpu_render_state.adapter,
            device: self.wgpu_render_state.device,
            queue: self.wgpu_render_state.queue,
        };

        engine.world().insert_resource(window_handle);
        engine.world().insert_resource(wgpu_handles);
        engine.world().insert_resource(egui_texture_target);
        engine.world().insert_resource(InputState::default());

        engine.build_plugins();

        engine.setup();
    }
}

#[derive(Resource)]
pub struct EguiTextureTarget {
    texture_id: eframe::egui::TextureId,
    texture_view: wgpu::TextureView,
    width: u32,
    height: u32,
}

impl EguiTextureTarget {
    pub fn new(
        wgpu_render_state: &eframe::egui_wgpu::RenderState,
        mut width: u32,
        mut height: u32,
    ) -> Self {
        width = width.max(1);
        height = height.max(1);

        let size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let texture = wgpu_render_state
            .device
            .create_texture(&wgpu::TextureDescriptor {
                label: Some("Egui Render Target"),
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu_render_state.target_format,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                    | wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            });

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let texture_id = wgpu_render_state.renderer.write().register_native_texture(
            &wgpu_render_state.device,
            &texture_view,
            wgpu::FilterMode::Linear,
        );

        Self {
            texture_id,
            texture_view,
            width,
            height,
        }
    }

    pub fn texture_id(&self) -> eframe::egui::TextureId {
        self.texture_id
    }

    pub fn texture_view(&self) -> &wgpu::TextureView {
        &self.texture_view
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn resize(
        &mut self,
        wgpu_render_state: &eframe::egui_wgpu::RenderState,
        width: u32,
        height: u32,
    ) {
        let width = width.max(1);
        let height = height.max(1);

        if self.width == width && self.height == height {
            return;
        }

        self.width = width;
        self.height = height;

        let size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let texture = wgpu_render_state
            .device
            .create_texture(&wgpu::TextureDescriptor {
                label: Some("Egui Render Target"),
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu_render_state.target_format,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                    | wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            });

        self.texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        wgpu_render_state
            .renderer
            .write()
            .update_egui_texture_from_wgpu_texture(
                &wgpu_render_state.device,
                &self.texture_view,
                wgpu::FilterMode::Linear,
                self.texture_id,
            );
    }

    fn render_target(&self) -> RenderTarget {
        RenderTarget::Texture(self.texture_view.clone())
    }
}

use eframe::egui::{self, Widget};

pub struct Viewport<'a> {
    engine: &'a mut Engine<EguiRunner>,
    render_state: &'a eframe::egui_wgpu::RenderState,
    size: egui::Vec2,
}

impl<'a> Viewport<'a> {
    pub fn new(
        engine: &'a mut Engine<EguiRunner>,
        frame: &'a eframe::Frame,
        size: egui::Vec2,
    ) -> Self {
        Self {
            engine,
            render_state: frame.wgpu_render_state().unwrap(),
            size,
        }
    }

    fn update_input_state(&mut self, ui: &eframe::egui::Ui, response: &eframe::egui::Response) {
        let mut input_state = self.engine.world().resource_mut::<InputState>();

        input_state.clear_frame_state();

        ui.input(|input| {
            for event in &input.events {
                match event {
                    eframe::egui::Event::Key {
                        key,
                        pressed,
                        repeat,
                        ..
                    } => {
                        if let Ok(mapped_key) = KeyboardKey::try_from(*key) {
                            if *pressed {
                                if !*repeat {
                                    input_state.keyboard.just_pressed_keys.insert(mapped_key);
                                    input_state.keyboard.held_keys.insert(mapped_key);
                                }
                            } else {
                                input_state.keyboard.held_keys.remove(&mapped_key);
                                input_state.keyboard.just_released_keys.insert(mapped_key);
                            }
                        }
                    }
                    eframe::egui::Event::PointerButton {
                        button, pressed, ..
                    } => {
                        let mouse_button = match button {
                            eframe::egui::PointerButton::Primary => crate::input::MouseButton::Left,
                            eframe::egui::PointerButton::Secondary => {
                                crate::input::MouseButton::Right
                            }
                            eframe::egui::PointerButton::Middle => {
                                crate::input::MouseButton::Middle
                            }
                            eframe::egui::PointerButton::Extra1 => crate::input::MouseButton::Back,
                            eframe::egui::PointerButton::Extra2 => {
                                crate::input::MouseButton::Forward
                            }
                        };

                        if *pressed {
                            input_state.mouse.just_pressed_buttons.insert(mouse_button);
                            input_state.mouse.held_buttons.insert(mouse_button);
                        } else {
                            input_state.mouse.held_buttons.remove(&mouse_button);
                            input_state.mouse.just_released_buttons.insert(mouse_button);
                        }
                    }
                    eframe::egui::Event::MouseWheel { delta, .. } => {
                        input_state.mouse.scroll_delta += glam::vec2(delta.x, delta.y);
                    }
                    eframe::egui::Event::MouseMoved(delta) => {
                        input_state.mouse.cursor.delta += glam::vec2(delta.x, delta.y);
                    }
                    _ => {}
                }
            }
        });

        if let Some(hover_pos) = response.hover_pos() {
            let local_pos = hover_pos - response.rect.min;
            input_state.mouse.cursor.position = glam::vec2(local_pos.x, local_pos.y);
        }
    }

    fn update_render_target(
        &mut self,
        _ui: &eframe::egui::Ui,
        response: &eframe::egui::Response,
    ) -> eframe::egui::TextureId {
        let width = response.rect.width().max(1.0) as u32;
        let height = response.rect.height().max(1.0) as u32;

        let mut target = self
            .engine
            .world()
            .get_resource_mut::<EguiTextureTarget>()
            .unwrap();
        target.resize(self.render_state, width, height);
        let texture_id = target.texture_id();
        let texture_view = target.texture_view().clone();

        let mut wgpu_handles = self
            .engine
            .world()
            .get_resource_mut::<WgpuHandles>()
            .unwrap();

        wgpu_handles.render_target = RenderTarget::Texture(texture_view.clone());

        texture_id
    }
}

impl<'a> Widget for Viewport<'a> {
    fn ui(mut self, ui: &mut egui::Ui) -> egui::Response {
        let (rect, response) = ui.allocate_exact_size(self.size, egui::Sense::click_and_drag());

        self.update_input_state(ui, &response);
        let texture_id = self.update_render_target(ui, &response);

        self.engine.update();

        let world = self.engine.world();
        let input_state = world.resource::<InputState>();
        let window_handle = world.resource::<WindowHandle>();

        window_handle.set_cursor_visible(input_state.mouse.cursor.visible);

        let _ = window_handle.set_cursor_grab(if input_state.mouse.cursor.locked {
            winit::window::CursorGrabMode::Locked
        } else {
            winit::window::CursorGrabMode::None
        });

        if !input_state.mouse.cursor.visible {
            ui.ctx().set_cursor_icon(eframe::egui::CursorIcon::None);
        }

        ui.put(
            rect,
            egui::Image::new(egui::load::SizedTexture::new(texture_id, rect.size())),
        );

        response
    }
}

impl TryFrom<egui::Key> for KeyboardKey {
    type Error = ();

    fn try_from(key: egui::Key) -> Result<Self, Self::Error> {
        let mapped = match key {
            egui::Key::A => KeyboardKey::A,
            egui::Key::B => KeyboardKey::B,
            egui::Key::C => KeyboardKey::C,
            egui::Key::D => KeyboardKey::D,
            egui::Key::E => KeyboardKey::E,
            egui::Key::F => KeyboardKey::F,
            egui::Key::G => KeyboardKey::G,
            egui::Key::H => KeyboardKey::H,
            egui::Key::I => KeyboardKey::I,
            egui::Key::J => KeyboardKey::J,
            egui::Key::K => KeyboardKey::K,
            egui::Key::L => KeyboardKey::L,
            egui::Key::M => KeyboardKey::M,
            egui::Key::N => KeyboardKey::N,
            egui::Key::O => KeyboardKey::O,
            egui::Key::P => KeyboardKey::P,
            egui::Key::Q => KeyboardKey::Q,
            egui::Key::R => KeyboardKey::R,
            egui::Key::S => KeyboardKey::S,
            egui::Key::T => KeyboardKey::T,
            egui::Key::U => KeyboardKey::U,
            egui::Key::V => KeyboardKey::V,
            egui::Key::W => KeyboardKey::W,
            egui::Key::X => KeyboardKey::X,
            egui::Key::Y => KeyboardKey::Y,
            egui::Key::Z => KeyboardKey::Z,

            egui::Key::Num0 => KeyboardKey::Zero,
            egui::Key::Num1 => KeyboardKey::One,
            egui::Key::Num2 => KeyboardKey::Two,
            egui::Key::Num3 => KeyboardKey::Three,
            egui::Key::Num4 => KeyboardKey::Four,
            egui::Key::Num5 => KeyboardKey::Five,
            egui::Key::Num6 => KeyboardKey::Six,
            egui::Key::Num7 => KeyboardKey::Seven,
            egui::Key::Num8 => KeyboardKey::Eight,
            egui::Key::Num9 => KeyboardKey::Nine,

            egui::Key::ArrowDown => KeyboardKey::DownArrow,
            egui::Key::ArrowLeft => KeyboardKey::LeftArrow,
            egui::Key::ArrowRight => KeyboardKey::RightArrow,
            egui::Key::ArrowUp => KeyboardKey::UpArrow,
            egui::Key::Escape => KeyboardKey::Escape,
            egui::Key::Tab => KeyboardKey::Tab,
            egui::Key::Backspace => KeyboardKey::Backspace,
            egui::Key::Enter => KeyboardKey::Enter,
            egui::Key::Space => KeyboardKey::Space,
            egui::Key::Insert => KeyboardKey::Insert,
            egui::Key::Delete => KeyboardKey::Delete,
            egui::Key::Home => KeyboardKey::Home,
            egui::Key::End => KeyboardKey::End,
            egui::Key::PageUp => KeyboardKey::PageUp,
            egui::Key::PageDown => KeyboardKey::PageDown,

            egui::Key::CapsLock => KeyboardKey::CapsLock,
            egui::Key::ShiftLeft => KeyboardKey::LeftShift,
            egui::Key::ShiftRight => KeyboardKey::RightShift,
            egui::Key::ControlLeft => KeyboardKey::LeftControl,
            egui::Key::ControlRight => KeyboardKey::RightControl,
            egui::Key::AltLeft => KeyboardKey::LeftAlt,
            egui::Key::AltRight => KeyboardKey::RightAlt,
            egui::Key::SuperLeft => KeyboardKey::LeftSuper,
            egui::Key::SuperRight => KeyboardKey::RightSuper,

            egui::Key::F1 => KeyboardKey::F1,
            egui::Key::F2 => KeyboardKey::F2,
            egui::Key::F3 => KeyboardKey::F3,
            egui::Key::F4 => KeyboardKey::F4,
            egui::Key::F5 => KeyboardKey::F5,
            egui::Key::F6 => KeyboardKey::F6,
            egui::Key::F7 => KeyboardKey::F7,
            egui::Key::F8 => KeyboardKey::F8,
            egui::Key::F9 => KeyboardKey::F9,
            egui::Key::F10 => KeyboardKey::F10,
            egui::Key::F11 => KeyboardKey::F11,
            egui::Key::F12 => KeyboardKey::F12,
            egui::Key::F13 => KeyboardKey::F13,
            egui::Key::F14 => KeyboardKey::F14,
            egui::Key::F15 => KeyboardKey::F15,
            egui::Key::F16 => KeyboardKey::F16,
            egui::Key::F17 => KeyboardKey::F17,
            egui::Key::F18 => KeyboardKey::F18,
            egui::Key::F19 => KeyboardKey::F19,
            egui::Key::F20 => KeyboardKey::F20,
            egui::Key::F21 => KeyboardKey::F21,
            egui::Key::F22 => KeyboardKey::F22,
            egui::Key::F23 => KeyboardKey::F23,
            egui::Key::F24 => KeyboardKey::F24,

            egui::Key::Backtick => KeyboardKey::Grave,
            egui::Key::Minus => KeyboardKey::Minus,
            egui::Key::Equals => KeyboardKey::Equal,
            egui::Key::OpenBracket => KeyboardKey::LeftBracket,
            egui::Key::CloseBracket => KeyboardKey::RightBracket,
            egui::Key::Backslash => KeyboardKey::Backslash,
            egui::Key::Semicolon => KeyboardKey::Semicolon,
            egui::Key::Quote => KeyboardKey::Apostrophe,
            egui::Key::Comma => KeyboardKey::Comma,
            egui::Key::Period => KeyboardKey::Period,
            egui::Key::Slash => KeyboardKey::Slash,

            egui::Key::NumLock => KeyboardKey::NumLock,
            egui::Key::ScrollLock => KeyboardKey::ScrollLock,
            egui::Key::PrintScreen => KeyboardKey::PrintScreen,
            egui::Key::Pause => KeyboardKey::Pause,

            _ => return Err(()),
        };

        Ok(mapped)
    }
}
