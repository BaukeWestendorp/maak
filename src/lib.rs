pub mod context;
pub mod window;

pub mod prelude {
    pub use crate::context::*;
    pub use crate::window::*;
    pub use ::glam::*;
}

pub use ::glam::*;
