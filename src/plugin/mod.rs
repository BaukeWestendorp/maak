use crate::{engine::Engine, runner::Runner};

pub mod render;
pub mod time;

pub trait Plugin<R: Runner> {
    fn build(&self, engine: &mut Engine<R>);
}
