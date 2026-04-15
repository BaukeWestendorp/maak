use crate::ecs::EntityHandle;
use crate::engine::{Backend, Scene};

pub mod camera;
pub mod camera_controller;
pub mod transform;

pub use camera::*;
pub use camera_controller::*;
pub use transform::*;

pub trait Component<B: Backend> {
    fn as_any(&self) -> &dyn std::any::Any;

    fn setup(&mut self, _entity: EntityHandle, _scene: &Scene<B>, _cx: &mut B) {}

    fn update(&mut self, _delta_time: f32, _entity: EntityHandle, _scene: &Scene<B>, _cx: &mut B) {}

    fn shutdown(&mut self, _entity: EntityHandle, _scene: &Scene<B>, _cx: &mut B) {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ComponentHandle(uuid::Uuid);

impl ComponentHandle {
    pub(crate) fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }

    pub fn as_uuid(&self) -> uuid::Uuid {
        self.0
    }
}

pub trait Bundle<B: Backend> {
    fn into_components(self) -> Vec<Box<dyn Component<B>>>;
}

macro_rules! impl_bundle_tuple {
    ($($name:ident),+) => {
        #[allow(non_snake_case)]
        impl<B: Backend + 'static, $($name: Component<B> + 'static),+> Bundle<B> for ($($name,)+) {
            fn into_components(self) -> Vec<Box<dyn Component<B>>> {
                let ($($name,)+) = self;
                vec![$(Box::new($name) as Box<dyn Component<B>>),+]
            }
        }
    };
}

impl_bundle_tuple!(A);
impl_bundle_tuple!(A, Be);
impl_bundle_tuple!(A, Be, C);
impl_bundle_tuple!(A, Be, C, D);
impl_bundle_tuple!(A, Be, C, D, E);
impl_bundle_tuple!(A, Be, C, D, E, F);
impl_bundle_tuple!(A, Be, C, D, E, F, G);
impl_bundle_tuple!(A, Be, C, D, E, F, G, H);
impl_bundle_tuple!(A, Be, C, D, E, F, G, H, I);
impl_bundle_tuple!(A, Be, C, D, E, F, G, H, I, J);
