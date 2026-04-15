use crate::ecs::Component;
use crate::engine::Backend;

#[derive(Debug)]
pub struct Transform {
    pub affine: glam::Affine3A,
}

impl Transform {
    pub fn identity() -> Self {
        Self { affine: glam::Affine3A::IDENTITY }
    }

    pub fn from_translation(translation: glam::Vec3) -> Self {
        Self { affine: glam::Affine3A::from_translation(translation) }
    }

    pub fn from_rotation(rotation: glam::Quat) -> Self {
        Self { affine: glam::Affine3A::from_quat(rotation) }
    }

    pub fn from_scale(scale: glam::Vec3) -> Self {
        Self { affine: glam::Affine3A::from_scale(scale) }
    }

    pub fn from_trs(translation: glam::Vec3, rotation: glam::Quat, scale: glam::Vec3) -> Self {
        Self {
            affine: glam::Affine3A::from_scale_rotation_translation(scale, rotation, translation),
        }
    }

    pub fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        Self::from_translation(glam::Vec3::new(x, y, z))
    }

    pub fn translation(&self) -> glam::Vec3A {
        self.affine.translation
    }

    pub fn with_translation(mut self, translation: glam::Vec3A) -> Self {
        self.affine.translation = translation;
        self
    }

    pub fn with_rotation(self, rotation: glam::Quat) -> Self {
        let t = self.translation();
        let s = self.scale();
        Self::from_trs(t.into(), rotation, s)
    }

    pub fn with_scale(self, scale: glam::Vec3) -> Self {
        let t = self.translation();
        let r = self.rotation();
        Self::from_trs(t.into(), r, scale)
    }

    pub fn set_translation(&mut self, translation: glam::Vec3A) -> &mut Self {
        self.affine.translation = translation;
        self
    }

    pub fn set_rotation(&mut self, rotation: glam::Quat) -> &mut Self {
        let t = self.translation();
        let s = self.scale();
        self.affine = glam::Affine3A::from_scale_rotation_translation(s, rotation, t.into());
        self
    }

    pub fn set_scale(&mut self, scale: glam::Vec3) -> &mut Self {
        let t = self.translation();
        let r = self.rotation();
        self.affine = glam::Affine3A::from_scale_rotation_translation(scale, r, t.into());
        self
    }

    pub fn translate(&mut self, offset: glam::Vec3A) -> &mut Self {
        self.affine.translation += offset;
        self
    }

    pub fn rotate(&mut self, offset: glam::Quat) -> &mut Self {
        let new_rot = offset * self.rotation();
        self.set_rotation(new_rot)
    }

    pub fn scale_by(&mut self, factor: glam::Vec3) -> &mut Self {
        let new_scale = self.scale() * factor;
        self.set_scale(new_scale)
    }

    pub fn scale(&self) -> glam::Vec3 {
        self.affine.matrix3.determinant().abs().powf(1.0 / 3.0) * glam::Vec3::ONE
    }

    pub fn rotation(&self) -> glam::Quat {
        glam::Quat::from_affine3a(&self.affine)
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self { affine: glam::Affine3A::default() }
    }
}

impl<B: Backend> Component<B> for Transform {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
