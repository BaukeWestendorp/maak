pub mod backend;
pub mod input;
pub mod scene;

pub use backend::*;
pub use input::*;
pub use scene::*;

pub struct Engine<B: Backend> {
    backend: B,

    scene: Scene<B>,
}

impl<B: Backend> Engine<B> {
    pub fn new(backend: B, scene: Scene<B>) -> Self {
        Self { scene, backend }
    }

    pub fn run(&mut self) {
        self.backend.run_scene(&mut self.scene);
    }
}
