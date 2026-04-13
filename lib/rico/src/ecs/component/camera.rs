use raylib::camera::Camera3D;
use raylib::texture::RenderTexture2D;

use crate::prelude::*;

pub struct Camera {
    camera: Camera3D,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            camera: Camera3D::perspective(
                raylib::math::Vector3::new(0.0, 4.0, -10.0),
                raylib::math::Vector3::forward(),
                raylib::math::Vector3::up(),
                90.0,
            ),
        }
    }
}

impl Component for Camera {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn update(&mut self) {
        // let mut d = rl.begin_drawing(rl_thread);

        // d.clear_background(Color::RAYWHITE);

        // for transform in cx.query::<crate::prelude::Transform>() {
        //     d.draw_mode3D(*camera, |mut d, _| {
        //         let center = transform.translation();
        //         let pos_x = (center + glam::Vec3A::new(1.0, 0.0, 0.0)).to_rl();
        //         let pos_y = (center + glam::Vec3A::new(0.0, 1.0, 0.0)).to_rl();
        //         let pos_z = (center + glam::Vec3A::new(0.0, 0.0, 1.0)).to_rl();
        //         d.draw_line_3D(center.to_rl(), pos_x, Color::RED);
        //         d.draw_line_3D(center.to_rl(), pos_y, Color::GREEN);
        //         d.draw_line_3D(center.to_rl(), pos_z, Color::BLUE);
        //     });
        // }

        // d.draw_fps(10, 10);
    }
}
