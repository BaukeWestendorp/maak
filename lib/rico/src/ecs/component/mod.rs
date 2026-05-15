use crate::engine::{Backend, Scene};

pub mod camera;
pub mod camera_controller;
pub mod transform;

pub use camera::*;
pub use camera_controller::*;
pub use transform::*;

pub trait Component<B: Backend> {
    fn setup(_entity: hecs::Entity, _scene: &Scene<B>, _cx: &mut B) {}

    fn update(_delta_time: f32, _entity: hecs::Entity, _scene: &Scene<B>, _cx: &mut B) {}

    fn shutdown(_entity: hecs::Entity, _scene: &Scene<B>, _cx: &mut B) {}
}
