use std::sync::Arc;

use bevy_ecs::resource::Resource;

use crate::{
    engine::Engine,
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
}

impl<'a> Widget for Viewport<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let (rect, response) = ui.allocate_exact_size(self.size, egui::Sense::drag());

        let width = rect.width().max(1.0) as u32;
        let height = rect.height().max(1.0) as u32;

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

        self.engine.update();

        ui.put(
            rect,
            egui::Image::new(egui::load::SizedTexture::new(texture_id, rect.size())),
        );

        response
    }
}
