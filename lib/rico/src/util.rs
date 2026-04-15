pub trait IntoVector3 {
    fn to_rl(self) -> raylib::math::Vector3;
}

impl IntoVector3 for glam::Vec3 {
    #[inline]
    fn to_rl(self) -> raylib::math::Vector3 {
        raylib::math::Vector3 { x: self.x, y: self.y, z: self.z }
    }
}

impl IntoVector3 for glam::IVec3 {
    #[inline]
    fn to_rl(self) -> raylib::math::Vector3 {
        raylib::math::Vector3 { x: self.x as f32, y: self.y as f32, z: self.z as f32 }
    }
}

impl IntoVector3 for glam::UVec3 {
    #[inline]
    fn to_rl(self) -> raylib::math::Vector3 {
        raylib::math::Vector3 { x: self.x as f32, y: self.y as f32, z: self.z as f32 }
    }
}

impl IntoVector3 for glam::Vec3A {
    #[inline]
    fn to_rl(self) -> raylib::math::Vector3 {
        raylib::math::Vector3 { x: self.x as f32, y: self.y as f32, z: self.z as f32 }
    }
}

pub trait IntoVector2 {
    fn to_rl(self) -> raylib::math::Vector2;
}

impl IntoVector2 for glam::Vec2 {
    #[inline]
    fn to_rl(self) -> raylib::math::Vector2 {
        raylib::math::Vector2 { x: self.x, y: self.y }
    }
}

impl IntoVector2 for glam::IVec2 {
    #[inline]
    fn to_rl(self) -> raylib::math::Vector2 {
        raylib::math::Vector2 { x: self.x as f32, y: self.y as f32 }
    }
}

impl IntoVector2 for glam::UVec2 {
    #[inline]
    fn to_rl(self) -> raylib::math::Vector2 {
        raylib::math::Vector2 { x: self.x as f32, y: self.y as f32 }
    }
}

pub trait ToGlamVector3 {
    fn to_glam(self) -> glam::Vec3;
}

impl ToGlamVector3 for raylib::math::Vector3 {
    #[inline]
    fn to_glam(self) -> glam::Vec3 {
        glam::Vec3::new(self.x, self.y, self.z)
    }
}

impl ToGlamVector3 for raylib::ffi::Vector3 {
    #[inline]
    fn to_glam(self) -> glam::Vec3 {
        glam::Vec3::new(self.x, self.y, self.z)
    }
}

pub trait ToGlamVector2 {
    fn to_glam(self) -> glam::Vec2;
}

impl ToGlamVector2 for raylib::math::Vector2 {
    #[inline]
    fn to_glam(self) -> glam::Vec2 {
        glam::Vec2::new(self.x, self.y)
    }
}

impl ToGlamVector2 for raylib::ffi::Vector2 {
    #[inline]
    fn to_glam(self) -> glam::Vec2 {
        glam::Vec2::new(self.x, self.y)
    }
}
