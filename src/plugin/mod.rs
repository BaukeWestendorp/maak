use crate::app::App;

pub mod render;
pub mod time;
pub mod window;

pub trait Plugin {
    fn build(&self, app: &mut App);
}
