use maak::prelude::*;

fn main() {
    let mut cx = Context::new(WIDTH, HEIGHT).expect("should open window");

    cx.window_mut().set_target_fps(0);

    let mut camera = Camera {
        position: Vec3::new(5.0, 5.0, -7.50),
        projection: Projection::Perspective { fov: 90.0 },
    };

    let mut last_frame_time = std::time::Instant::now();
    while cx.window().is_open() {
        let t_start = std::time::Instant::now();
        let dt = t_start.duration_since(last_frame_time).as_secs_f32();

        draw(&mut camera, dt, &mut cx);
        let t_end = std::time::Instant::now();
        let duration = t_end.duration_since(t_start);
        let fps = 1.0 / duration.as_secs_f32();
        if last_frame_time.elapsed().as_secs_f32() >= 0.1 {
            last_frame_time = std::time::Instant::now();
            cx.window_mut().set_title(&format!("Grapher - FPS: {:.2}", fps));
        }
    }
}

fn draw(camera: &mut Camera, _dt: f32, cx: &mut Context) {
    let center = Vec3::new(0.0, 0.0, 0.0);
    let grid_size = 10.0;

    cx.set_color(0x404040);
    cx.draw_grid_3d(
        center + Vec3::new(grid_size, 0.0, grid_size) / 2.0,
        Vec2::ONE * grid_size,
        &camera,
    );
    cx.set_color(0xFF4040);
    cx.draw_line_3d(Vec3::new(0.0, 0.0, 0.0), Vec3::new(grid_size, 0.0, 0.0), &camera);
    cx.set_color(0x40FF40);
    cx.draw_line_3d(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, grid_size, 0.0), &camera);
    cx.set_color(0x4040FF);
    cx.draw_line_3d(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, grid_size), &camera);

    cx.set_color(0xa050a0);
    for x in -50..50 {
        for z in -50..50 {
            for y in -50..50 {
                let x_f = x as f32 * 0.2;
                let y_f = y as f32 * 0.2;
                let z_f = z as f32 * 0.2;

                let eq = (x_f * x_f + y_f * y_f + z_f * z_f - 100.0).abs() < 1.0;

                if eq {
                    cx.draw_point_3d(
                        Vec3::new(
                            (x as f32 + 50.0) / grid_size,
                            (y as f32 + 50.0) / grid_size,
                            (z as f32 + 50.0) / grid_size,
                        ),
                        &camera,
                    );
                }
            }
        }
    }

    cx.update_with_buffer().expect("should update with buffer");
    cx.buffer_mut().iter_mut().for_each(|p| *p = 0);
}

const WIDTH: usize = 640;
const HEIGHT: usize = 480;
