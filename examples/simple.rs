use std::path::Path;

use maak::{
    plugin::{
        render::{Camera, Material, Mesh, MeshRenderer, RenderPlugin, RenderState},
        time::{Time, TimePlugin},
        window::WindowPlugin,
    },
    prelude::*,
};

fn main() {
    let mut app = App::new();

    app.add_plugin(WindowPlugin {
        title: "Maak".to_string(),
        width: 800,
        height: 600,
    });
    app.add_plugin(TimePlugin);
    app.add_plugin(RenderPlugin);

    app.add_setup_system(setup_scene);
    app.add_update_system(rotate_cubes);

    app.run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<Material>>,
    render: Res<RenderState>,
) {
    let cube_mesh = meshes.add(Mesh::cube(&render));

    let custom_mat = materials.add(Material::new(
        &render,
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
    camera.translation = Vec3::new(0.0, 0.0, time.elapsed_seconds());
    // for mut transform in query.iter_mut() {
    //     transform.rotate_y(time.delta_seconds() * 2.0);
    // }
}
