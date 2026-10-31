use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

use bevy_ecs::resource::Resource;

use crate::engine::Engine;

#[cfg(feature = "egui")]
pub mod egui;

pub mod standard;

pub trait Runner {
    fn run(self, engine: &mut Engine<Self>)
    where
        Self: Sized;
}

pub enum RenderTarget {
    Surface(wgpu::Surface<'static>),
    Texture(wgpu::TextureView),
}

#[derive(Resource)]
pub struct WgpuHandles {
    pub render_target: RenderTarget,
    pub target_format: wgpu::TextureFormat,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

#[derive(Resource)]
pub struct WindowHandle(Arc<winit::window::Window>);

impl Deref for WindowHandle {
    type Target = Arc<winit::window::Window>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for WindowHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
