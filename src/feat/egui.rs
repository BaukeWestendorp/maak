use bevy_ecs::resource::Resource;
use eframe::egui_wgpu;

use crate::{
    app::App,
    plugin::{
        Plugin,
        render::{OffscreenRenderTarget, RenderState},
    },
};

pub struct EguiPlugin {
    wgpu_render_state: egui_wgpu::RenderState,
}

impl EguiPlugin {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        let wgpu_render_state = cc
            .wgpu_render_state
            .clone()
            .expect("wgpu_render_state is required for Maak to work");

        Self { wgpu_render_state }
    }
}

impl Plugin for EguiPlugin {
    fn build(&self, app: &mut App) {
        let device = self.wgpu_render_state.device.clone();
        let queue = self.wgpu_render_state.queue.clone();
        let surface_format = self.wgpu_render_state.target_format;

        let camera_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Camera Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        app.world().insert_resource(RenderState {
            surface: None,
            surface_format,
            adapter: self.wgpu_render_state.adapter.clone(),
            device: device.clone(),
            queue,
            camera_layout,
        });
        let size = wgpu::Extent3d {
            width: 1080,
            height: 720,
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Engine Offscreen Texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: surface_format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let egui_tex_id = self
            .wgpu_render_state
            .renderer
            .write()
            .register_native_texture(&device, &view, wgpu::FilterMode::Linear);

        app.world().insert_resource(OffscreenRenderTarget { view });

        app.world()
            .insert_resource(EguiTextureTarget { egui_tex_id });
    }
}

#[derive(Resource)]
pub struct EguiTextureTarget {
    pub egui_tex_id: eframe::egui::TextureId,
}
