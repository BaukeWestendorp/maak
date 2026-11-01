use crate::{engine::Engine, runner::Runner};

pub mod camera_control;
pub mod render;
pub mod time;
pub mod transform;

pub trait Plugin<R: Runner> {
    fn build(&self, engine: &mut Engine<R>);
}
