#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // Hide console window on Windows in release.

use std::{path::Path, sync::Arc};

use maak::{
    plugin::{
        render::{BindGroupLayouts, Camera, Material, Mesh, MeshRenderer, RenderPlugin},
        time::{Time, TimePlugin},
    },
    prelude::*,
    runner::{
        WgpuHandles,
        egui::{EguiRunner, Viewport},
    },
};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default().with_inner_size([1080.0, 720.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Maak egui",
        options,
        Box::new(|cc| Ok(Box::new(Application::new(cc)))),
    )
}

struct Application {
    engine: maak::engine::Engine<EguiRunner>,
}

impl Application {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        let winit_window = Arc::clone(cc.winit_window().unwrap());
        let wgpu_render_state = cc.wgpu_render_state.clone().unwrap();
        let mut engine = Engine::new(EguiRunner::new(winit_window, wgpu_render_state));

        engine.add_plugin(TimePlugin);
        engine.add_plugin(RenderPlugin);

        engine.add_setup_system(setup_scene);
        engine.add_update_system(rotate_cubes);

        engine.run();

        Self { engine }
    }
}

impl eframe::App for Application {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, frame: &mut eframe::Frame) {
        eframe::egui::CentralPanel::default().show(ui, |ui| {
            ui.add(Viewport::new(&mut self.engine, frame, ui.available_size()));
        });

        ui.request_repaint();
    }
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
        Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/egui.wgsl"),
    ));

    commands
        .spawn((Name::new("Parent Entity"),))
        .with_children(|parent| {
            parent.spawn((MeshRenderer {
                mesh: cube_mesh.clone(),
                material: custom_mat.clone(),
            },));
        });
}

fn rotate_cubes(time: Res<Time>, mut camera: ResMut<Camera>) {
    camera.translation = Vec3::new(0.0, 0.0, time.elapsed_seconds() * 5.0);
}
