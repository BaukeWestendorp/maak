use maak::prelude::*;

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

fn main() -> Result<()> {
    let mut cx = Context::new(WIDTH, HEIGHT).expect("should open window");

    cx.window_mut().set_target_fps(0);

    cx.camera_mut().translate_to(Vec3::new(6.0, 6.0, -6.0));
    cx.camera_mut().rotate_by_euler_angles(30f32.to_radians(), -45f32.to_radians(), 0.0);

    let mut axis = cx.spawn(|_| Axis::default());
    let mut grid = cx.spawn(|_| Grid::default());
    let mut visualization = cx.spawn(|_| Visualization::default());

    while cx.window().is_open() {
        cx.clear_buffer();

        let (mouse_x, _) = cx.window().get_mouse_pos(MouseMode::Pass).unwrap_or_default();
        let w = cx.width();
        let rot = mouse_x / w as f32 * std::f32::consts::TAU;
        axis.set_euler_angles(0.0, rot, 0.0);
        grid.set_euler_angles(0.0, rot, 0.0);
        visualization.set_euler_angles(0.0, rot, 0.0);

        grid.draw(&mut cx);
        axis.draw(&mut cx);
        visualization.draw(&mut cx);

        cx.update_with_buffer()?;
    }

    Ok(())
}

struct Axis {
    pub divisions: usize,
}

impl Default for Axis {
    fn default() -> Self {
        Self { divisions: 5 }
    }
}

impl Drawable for Axis {
    fn on_draw(&mut self, cx: &mut Context) {
        cx.push_color(0xFF4040);
        cx.draw_line_3d(Vec3::NEG_X * self.divisions as f32, Vec3::X * self.divisions as f32);
        cx.pop_color();

        cx.push_color(0x40FF40);
        cx.draw_line_3d(Vec3::NEG_Y * self.divisions as f32, Vec3::Y * self.divisions as f32);
        cx.pop_color();

        cx.push_color(0x4040FF);
        cx.draw_line_3d(Vec3::NEG_Z * self.divisions as f32, Vec3::Z * self.divisions as f32);
        cx.pop_color();
    }
}

struct Grid {
    pub size: usize,
}

impl Default for Grid {
    fn default() -> Self {
        Self { size: 5 }
    }
}

impl Drawable for Grid {
    fn on_draw(&mut self, cx: &mut Context) {
        let min_x = -(self.size as f32);
        let max_x = self.size as f32;
        let min_z = -(self.size as f32);
        let max_z = self.size as f32;

        cx.set_color(0x404040);

        for x in (min_x as isize)..=(max_x as isize) {
            let from = Vec3::new(x as f32, 0.0, min_z);
            let to = Vec3::new(x as f32, 0.0, max_z);
            cx.draw_line_3d(from, to);
        }

        for z in (min_z as isize)..=(max_z as isize) {
            let from = Vec3::new(min_x, 0.0, z as f32);
            let to = Vec3::new(max_x, 0.0, z as f32);
            cx.draw_line_3d(from, to);
        }
    }
}

struct Visualization {
    pub divisions: usize,
    pub subdivisions: f32,
}

impl Default for Visualization {
    fn default() -> Self {
        Self { divisions: 5, subdivisions: 25.0 }
    }
}

impl Drawable for Visualization {
    fn on_draw(&mut self, cx: &mut Context) {
        cx.push_color(0xa050a0);
        let step_size = 1.0 / self.subdivisions as f32;
        let half_steps = (self.divisions as f32 * self.subdivisions) as isize;
        let x_steps = -half_steps..=half_steps;
        let y_steps = -half_steps..=half_steps;
        let z_steps = -half_steps..=half_steps;

        let threshold = step_size * 0.8;

        for x_step in x_steps {
            for y_step in y_steps.clone() {
                for z_step in z_steps.clone() {
                    let x = x_step as f32 * step_size;
                    let y = y_step as f32 * step_size;
                    let z = z_step as f32 * step_size;

                    let eq = (x * x + y * -y + z * z - 12.5).abs() < threshold;

                    if eq {
                        cx.draw_point_3d(Vec3::new(x, y, z));
                    }
                }
            }
        }
        cx.pop_color();
    }
}
