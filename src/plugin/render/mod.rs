use bevy_ecs::resource::Resource;
use bevy_ecs::system::{Query, Res, ResMut};

use crate::runner::WindowHandle;
use crate::{
    asset::Assets,
    engine::Engine,
    plugin::Plugin,
    runner::{Runner, WgpuHandles},
};

mod camera;
mod material;
mod mesh;
mod uniform;

pub use self::camera::*;
pub use self::material::*;
pub use self::mesh::*;
pub use self::uniform::*;

pub struct RenderPlugin;

impl<R: Runner> Plugin<R> for RenderPlugin {
    fn build(&self, engine: &mut Engine<R>) {
        let world = engine.world();

        let aspect_ratio = {
            let window = world.get_resource::<WindowHandle>().unwrap();
            window.inner_size().width as f32 / window.inner_size().height as f32
        };
        world.insert_resource(Camera::new(aspect_ratio));

        world.insert_resource(Assets::<Mesh>::new());
        world.insert_resource(Assets::<Material>::new());

        let wgpu_handles = world.resource::<WgpuHandles>();

        let camera_bind_group_layout =
            wgpu_handles
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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

        world.insert_resource(CameraBinding::new(wgpu_handles, &camera_bind_group_layout));

        engine.world().insert_resource(BindGroupLayouts {
            camera: camera_bind_group_layout,
        });

        engine.add_update_system(render_system);
    }
}

fn render_system(
    wgpu_handles: Res<WgpuHandles>,
    window_handle: Res<WindowHandle>,
    mut camera: ResMut<Camera>,
    camera_binding: Res<CameraBinding>,
    meshes: Res<Assets<Mesh>>,
    materials: Res<Assets<Material>>,
    renderables: Query<&MeshRenderer>,
) {
    let aspect_ratio = {
        let window = window_handle;
        window.inner_size().width as f32 / window.inner_size().height as f32
    };
    camera.aspect_ratio = aspect_ratio;

    let camera_uniform_data = CameraUniformData::new(&camera);
    camera_binding
        .uniform
        .write(&wgpu_handles, &camera_uniform_data);

    let mut frame_to_present = None;
    let surface_view;
    let view = match &wgpu_handles.render_target {
        crate::runner::RenderTarget::Surface(surface) => {
            let frame = match surface.get_current_texture() {
                wgpu::CurrentSurfaceTexture::Success(surface_texture) => surface_texture,
                wgpu::CurrentSurfaceTexture::Suboptimal(surface_texture) => surface_texture,
                _ => {
                    log::warn!("Failed to acquire next swap chain texture!");
                    return;
                }
            };
            surface_view = frame
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());
            frame_to_present = Some(frame);
            &surface_view
        }
        crate::runner::RenderTarget::Texture(tex_view) => tex_view,
    };

    let mut encoder = wgpu_handles
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
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
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

    wgpu_handles.queue.submit(Some(encoder.finish()));

    if let Some(frame) = frame_to_present {
        wgpu_handles.queue.present(frame);
    }
}

#[derive(Resource)]
pub struct BindGroupLayouts {
    pub camera: wgpu::BindGroupLayout,
}
