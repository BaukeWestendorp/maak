use std::path::Path;

use maak::{
    plugin::{
        camera_control::CameraControlPlugin,
        render::{Camera, Material, Mesh, MeshRenderer, RenderPlugin},
        time::{Time, TimePlugin},
        transform::{GlobalTransform, TransformPlugin},
    },
    prelude::*,
    runner::WgpuHandles,
};

fn main() {
    let mut engine = Engine::new(StandardRunner);

    engine.add_plugin(TimePlugin);
    engine.add_plugin(RenderPlugin);
    engine.add_plugin(TransformPlugin);
    engine.add_plugin(CameraControlPlugin);

    engine.add_setup_system(setup_scene);
    engine.add_update_system(rotate_cubes);

    engine.run();
}

fn setup_scene(
    camera: Res<Camera>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<Material>>,
    wgpu_handles: Res<WgpuHandles>,
) {
    let cube_mesh = meshes.add(Mesh::cube(&wgpu_handles));

    let custom_mat = materials.add(Material::new(
        &wgpu_handles,
        &camera,
        Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/simple.wgsl"),
    ));

    commands.spawn((
        GlobalTransform::default(),
        MeshRenderer {
            mesh: cube_mesh.clone(),
            material: custom_mat.clone(),
        },
    ));
}

fn rotate_cubes(time: Res<Time>, mut query: Query<&mut GlobalTransform, With<MeshRenderer>>) {
    for mut transform in query.iter_mut() {
        let mut rotation = transform.rotation();
        rotation = rotation * glam::Quat::from_rotation_y(0.5 * time.delta_seconds());
        transform.set_rotation(rotation);
    }
}
