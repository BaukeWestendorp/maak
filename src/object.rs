use crate::prelude::*;

pub struct Object<T> {
    value: T,

    pub affine: Affine3,
}

impl<T> Object<T> {
    pub fn with_affine(mut self, affine: Affine3) -> Self {
        self.set_affine(affine);
        self
    }

    pub fn set_affine(&mut self, affine: Affine3) {
        self.affine = affine;
    }

    pub fn affine_mut(&mut self) -> &mut Affine3 {
        &mut self.affine
    }

    pub fn set_translation(&mut self, translation: Vec3) {
        self.affine.translation = translation;
    }

    pub fn with_translation(mut self, translation: Vec3) -> Self {
        self.set_translation(translation);
        self
    }

    pub fn translate_to(&mut self, translation: Vec3) {
        self.set_translation(translation);
    }

    pub fn translate_by(&mut self, translation: Vec3) {
        self.affine.translation += translation;
    }

    pub fn translation(&self) -> Vec3 {
        self.affine.translation
    }

    pub fn set_rotation(&mut self, rotation: Quat) {
        let (scale, _, translation) = self.affine.to_scale_rotation_translation();
        self.affine = Affine3::from_scale_rotation_translation(scale, rotation, translation);
    }

    pub fn with_rotation(mut self, rotation: Quat) -> Self {
        self.set_rotation(rotation);
        self
    }

    pub fn rotate_to(&mut self, rotation: Quat) {
        self.set_rotation(rotation);
    }

    pub fn rotate_by(&mut self, rotation: Quat) {
        let (scale, current_rotation, translation) = self.affine.to_scale_rotation_translation();
        let new_rotation = rotation * current_rotation;
        self.affine = Affine3::from_scale_rotation_translation(scale, new_rotation, translation);
    }

    pub fn set_euler_angles(&mut self, pitch: f32, yaw: f32, roll: f32) {
        let rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
        self.set_rotation(rotation);
    }

    pub fn with_euler_angles(mut self, pitch: f32, yaw: f32, roll: f32) -> Self {
        self.set_euler_angles(pitch, yaw, roll);
        self
    }

    pub fn rotate_by_euler_angles(&mut self, pitch: f32, yaw: f32, roll: f32) {
        let delta_rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
        self.rotate_by(delta_rotation);
    }

    pub fn rotate_pitch(&mut self, angle: f32) {
        self.rotate_by(Quat::from_rotation_x(angle));
    }

    pub fn rotate_yaw(&mut self, angle: f32) {
        self.rotate_by(Quat::from_rotation_y(angle));
    }

    pub fn rotate_roll(&mut self, angle: f32) {
        self.rotate_by(Quat::from_rotation_z(angle));
    }

    pub fn rotation(&self) -> Quat {
        let (_, rotation, _) = self.affine.to_scale_rotation_translation();
        rotation
    }
}

impl<T> std::ops::DerefMut for Object<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T> std::ops::Deref for Object<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Context {
    pub fn spawn<T, F: FnOnce(&mut Self) -> T>(&mut self, f: F) -> Object<T> {
        Object { value: f(self), affine: Default::default() }
    }
}

pub trait Drawable {
    fn on_draw(&mut self, cx: &mut Context);
}

impl<T: Drawable> Object<T> {
    pub fn draw(&mut self, cx: &mut Context) {
        let (_, rotation, translation) = self.affine.to_scale_rotation_translation();
        cx.push_translation(translation);
        cx.push_rotation(rotation);
        self.on_draw(cx);
        cx.pop_rotation();
        cx.pop_translation();
    }
}
