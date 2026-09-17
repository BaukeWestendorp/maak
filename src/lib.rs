pub mod camera;
pub mod context;
pub mod object;
pub mod window;

pub mod prelude {
    pub use crate::camera::*;
    pub use crate::context::*;
    pub use crate::object::*;
    pub use crate::window::*;
    pub use ::glam::*;
}

pub use ::glam;
