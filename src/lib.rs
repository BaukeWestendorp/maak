pub mod prelude {
    pub use crate::asset::*;
    pub use crate::ecs::prelude::*;
    pub use crate::engine::*;
    pub use crate::math::prelude::*;
    pub use crate::plugin::*;
    pub use crate::runner::standard::StandardRunner;
}

pub mod asset;
pub mod engine;
pub mod plugin;
pub mod runner;
pub use ::bevy_ecs as ecs;
pub use ::glam as math;
