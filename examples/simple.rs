use std::path::Path;

use maak::{
    plugin::{
        render::{BindGroupLayouts, Camera, Material, Mesh, MeshRenderer, RenderPlugin},
        time::{Time, TimePlugin},
    },
    prelude::*,
    runner::WgpuHandles,
};

fn main() {
    let mut engine = Engine::new(StandardRunner);

    engine.add_plugin(TimePlugin);
    engine.add_plugin(RenderPlugin);

    engine.add_setup_system(setup_scene);
    engine.add_update_system(rotate_cubes);

    engine.run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<Material>>,
    wgpu_handles: Res<WgpuHandles>,
    bind_group_layouts: Res<BindGroupLayouts>,
) {
    let cube_mesh = meshes.add(Mesh::cube(&wgpu_handles));

    let custom_mat = materials.add(Material::new(
        &wgpu_handles,
        &bind_group_layouts,
        Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/simple.wgsl"),
    ));

    commands
        .spawn((
            // LocalTransform::from_translation(Vec3::new(0.0, 0.0, -5.0)),
            // GlobalTransform::default(),
            Name::new("Parent Entity"),
        ))
        .with_children(|parent| {
            parent.spawn((
                MeshRenderer {
                    mesh: cube_mesh.clone(),
                    material: custom_mat.clone(),
                },
                // LocalTransform::from_translation(Vec3::new(2.0, 0.0, 0.0)),
                // GlobalTransform::default(),
            ));
        });
}

fn rotate_cubes(
    time: Res<Time>,
    mut camera: ResMut<Camera>,
    // mut query: Query<&mut LocalTransform, With<MeshRenderer>>
) {
    camera.translation = Vec3::new(0.0, 0.0, time.elapsed_seconds() * 5.0);
    // for mut transform in query.iter_mut() {
    //     transform.rotate_y(time.delta_seconds() * 2.0);
    // }
}
