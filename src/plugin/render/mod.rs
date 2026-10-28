use std::sync::Arc;

use bevy_ecs::{
    resource::Resource,
    system::{Query, Res},
};

mod camera;
mod material;
mod mesh;
mod uniform;

pub use self::camera::*;
pub use self::material::*;
pub use self::mesh::*;
pub use self::uniform::*;

use crate::{
    app::App,
    asset::Assets,
    plugin::{Plugin, window::MainWindow},
};

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        if !app.world().contains_resource::<RenderState>() {
            let window = Arc::clone(&app.world().resource::<MainWindow>().0);
            let render_state = pollster::block_on(async { init_wgpu(window).await })
                .expect("Failed to initialize wgpu");
            app.world().insert_resource(render_state);
        }

        app.world().insert_resource(Assets::<Mesh>::new());
        app.world().insert_resource(Assets::<Material>::new());

        let world = app.world();
        let render_state = world.resource::<RenderState>();
        world.insert_resource(CameraBinding::new(render_state));

        app.add_update_system(render_system);
    }
}

async fn init_wgpu(window: Arc<winit::window::Window>) -> anyhow::Result<RenderState> {
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
            memory_hints: Default::default(),
            trace: wgpu::Trace::Off,
        })
        .await?;

    surface.configure(
        &device,
        &wgpu::SurfaceConfiguration {
            present_mode: wgpu::PresentMode::Immediate,
            ..surface
                .get_default_config(&adapter, width, height)
                .expect("Failed to get default surface configuration")
        },
    );

    let surface_format = surface
        .get_capabilities(&adapter)
        .formats
        .first()
        .cloned()
        .expect("Failed to get wgpu surface format");

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

    Ok(RenderState {
        surface: Some(surface),
        surface_format,
        adapter,
        device,
        queue,

        camera_layout,
    })
}

#[derive(Resource)]
pub struct RenderState {
    // FIXME: This Option probably could be a specific enum.
    pub surface: Option<wgpu::Surface<'static>>,
    pub surface_format: wgpu::TextureFormat,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,

    pub camera_layout: wgpu::BindGroupLayout,
}

#[derive(Resource)]
pub struct OffscreenRenderTarget {
    pub view: wgpu::TextureView,
}

fn render_system(
    render_state: Res<RenderState>,
    offscreen_target: Option<Res<OffscreenRenderTarget>>,
    camera: Res<Camera>,
    camera_binding: Res<CameraBinding>,
    meshes: Res<Assets<Mesh>>,
    materials: Res<Assets<Material>>,
    renderables: Query<&MeshRenderer>,
) {
    let view_proj = (camera.proj_matrix() * camera.view_matrix()).to_cols_array_2d();
    camera_binding
        .uniform
        .write(&render_state, &CameraUniformData { view_proj });

    let mut frame_to_present = None;

    let view = if let Some(surface) = &render_state.surface {
        let frame = match surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(surface_texture) => surface_texture,
            wgpu::CurrentSurfaceTexture::Suboptimal(surface_texture) => surface_texture,
            _ => {
                log::warn!("Failed to acquire next swap chain texture!");
                return;
            }
        };
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        frame_to_present = Some(frame);
        view
    } else if let Some(offscreen) = &offscreen_target {
        offscreen.view.clone()
    } else {
        return;
    };
    let mut encoder = render_state
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

    {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Main Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.1,
                        b: 0.1,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });

        pass.set_bind_group(0, camera_binding.uniform.bind_group(), &[]);

        for renderer in renderables.iter() {
            if let (Some(mesh), Some(material)) = (
                meshes.get(&renderer.mesh),
                materials.get(&renderer.material),
            ) {
                pass.set_pipeline(&material.pipeline);
                pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
                pass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                pass.draw_indexed(0..mesh.index_count, 0, 0..1);
            }
        }
    }

    render_state.queue.submit(Some(encoder.finish()));

    if let Some(frame) = frame_to_present {
        render_state.queue.present(frame);
    }
}
