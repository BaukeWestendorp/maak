pub mod prelude {
    pub use crate::app::*;
    pub use crate::asset::*;
    pub use crate::ecs::prelude::*;
    pub use crate::math::prelude::*;
    pub use crate::plugin::*;
}

pub mod app;
pub mod asset;
pub mod plugin;
pub use ::bevy_ecs as ecs;
pub use ::glam as math;

#[cfg(any(feature = "egui"))]
pub mod feat;
