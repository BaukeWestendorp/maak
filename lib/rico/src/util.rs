pub trait IntoVector3 {
    fn to_rl(self) -> raylib::ffi::Vector3;
}

impl IntoVector3 for glam::Vec3 {
    #[inline]
    fn to_rl(self) -> raylib::ffi::Vector3 {
        raylib::ffi::Vector3 { x: self.x, y: self.y, z: self.z }
    }
}

impl IntoVector3 for glam::IVec3 {
    #[inline]
    fn to_rl(self) -> raylib::ffi::Vector3 {
        raylib::ffi::Vector3 { x: self.x as f32, y: self.y as f32, z: self.z as f32 }
    }
}

impl IntoVector3 for glam::UVec3 {
    #[inline]
    fn to_rl(self) -> raylib::ffi::Vector3 {
        raylib::ffi::Vector3 { x: self.x as f32, y: self.y as f32, z: self.z as f32 }
    }
}

impl IntoVector3 for glam::Vec3A {
    #[inline]
    fn to_rl(self) -> raylib::ffi::Vector3 {
        raylib::ffi::Vector3 { x: self.x as f32, y: self.y as f32, z: self.z as f32 }
    }
}
