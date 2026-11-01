use bevy_ecs::component::Component;

use crate::{engine::Engine, plugin::Plugin, runner::Runner};

pub struct TransformPlugin;

impl<R: Runner> Plugin<R> for TransformPlugin {
    fn build(&self, _engine: &mut Engine<R>) {}
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Component)]
pub struct GlobalTransform {
    affine: glam::Affine3,
}

impl GlobalTransform {
    pub fn new(affine: glam::Affine3) -> Self {
        Self { affine }
    }

    pub fn with_translation(mut self, translation: glam::Vec3) -> Self {
        self.affine.translation = translation;
        self
    }

    pub fn with_rotation(mut self, rotation: glam::Quat) -> Self {
        let (scale, _, translation) = self.affine.to_scale_rotation_translation();
        self.affine = glam::Affine3::from_scale_rotation_translation(scale, rotation, translation);
        self
    }

    pub fn with_scale(mut self, scale: glam::Vec3) -> Self {
        let (_, rotation, translation) = self.affine.to_scale_rotation_translation();
        self.affine = glam::Affine3::from_scale_rotation_translation(scale, rotation, translation);
        self
    }

    pub fn affine(&self) -> glam::Affine3 {
        self.affine
    }

    pub fn translation(&self) -> glam::Vec3 {
        self.affine.translation
    }

    pub fn rotation(&self) -> glam::Quat {
        let (_, rotation, _) = self.affine.to_scale_rotation_translation();
        rotation
    }

    pub fn scale(&self) -> glam::Vec3 {
        let (scale, _, _) = self.affine.to_scale_rotation_translation();
        scale
    }

    pub fn set_translation(&mut self, translation: glam::Vec3) {
        self.affine.translation = translation;
    }

    pub fn set_rotation(&mut self, rotation: glam::Quat) {
        let (scale, _, _) = self.affine.to_scale_rotation_translation();
        self.affine = glam::Affine3::from_scale_rotation_translation(
            scale,
            rotation,
            self.affine.translation,
        );
    }

    pub fn set_scale(&mut self, scale: glam::Vec3) {
        let (_, rotation, _) = self.affine.to_scale_rotation_translation();
        self.affine = glam::Affine3::from_scale_rotation_translation(
            scale,
            rotation,
            self.affine.translation,
        );
    }
}
