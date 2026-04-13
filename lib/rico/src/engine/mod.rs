use crate::prelude::*;

pub mod scene;

pub struct Engine {
    scenes: slotmap::SlotMap<SceneHandle, (Box<dyn Scene>, SceneData)>,
}

impl Engine {
    pub fn new() -> Self {
        Self { scenes: slotmap::SlotMap::default() }
    }

    pub fn with_scene<S: Scene + 'static>(mut self, scene: S) -> Self {
        self.scenes.insert((Box::new(scene), SceneData::new()));
        self
    }

    pub fn run(&mut self) {
        let (mut rl, mut rl_thread) = raylib::init()
            .title("Rico Application")
            .size(1080, 720)
            .resizable()
            .vsync()
            .msaa_4x()
            .build();

        let mut rl_cx = RaylibContext { rl: &mut rl, rl_thread: &mut rl_thread };

        for (_handle, (scene, scene_data)) in self.scenes.iter_mut() {
            scene.setup(scene_data);

            // for all components do setup, update, shutdown.
        }

        while !rl_cx.rl.window_should_close() {
            for (_handle, (scene, scene_data)) in self.scenes.iter_mut() {
                scene.update(scene_data);
            }
        }

        for (_handle, (scene, scene_data)) in self.scenes.iter_mut() {
            scene.shutdown(scene_data);
        }
    }
}

pub(crate) struct RaylibContext<'rl> {
    pub rl: &'rl mut raylib::RaylibHandle,
    pub rl_thread: &'rl mut raylib::RaylibThread,
}
