use bevy_ecs::system::{Query, Res};

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

        let window_handle = world.get_resource::<WindowHandle>().unwrap();
        let wgpu_handles = world.resource::<WgpuHandles>();
        world.insert_resource(Camera::new(window_handle.aspect_ratio(), wgpu_handles));

        world.insert_resource(Assets::<Mesh>::new());
        world.insert_resource(Assets::<Material>::new());

        engine.add_update_system(camera::update);
        engine.add_update_system(self::update);
    }
}

fn update(
    wgpu_handles: Res<WgpuHandles>,
    camera: Res<Camera>,
    meshes: Res<Assets<Mesh>>,
    materials: Res<Assets<Material>>,
    renderables: Query<&MeshRenderer>,
) {
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

        pass.set_bind_group(0, camera.binding().uniform.bind_group(), &[]);

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
