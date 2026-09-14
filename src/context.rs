use crate::prelude::*;

pub struct Context {
    window: Window,
    buffer: Vec<u32>,

    color: u32,
}

impl Context {
    pub fn new(width: usize, height: usize) -> crate::window::Result<Self> {
        let window = Window::new("MAAK", width, height, WindowOptions::default())?;
        Ok(Self { window, buffer: vec![0u32; width * height], color: 0xffffff })
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn window_mut(&mut self) -> &mut Window {
        &mut self.window
    }

    pub fn buffer(&self) -> &[u32] {
        &self.buffer
    }

    pub fn buffer_mut(&mut self) -> &mut [u32] {
        &mut self.buffer
    }

    pub fn update_with_buffer(&mut self) -> crate::window::Result<()> {
        let (width, height) = self.window().get_size();
        self.window.update_with_buffer(&self.buffer, width, height)
    }

    pub fn set_color(&mut self, color: u32) {
        self.color = color;
    }
}

// Screen Space
impl Context {
    pub fn width(&self) -> usize {
        self.window.get_size().0
    }

    pub fn height(&self) -> usize {
        self.window.get_size().1
    }

    pub fn draw_pixel(&mut self, point: impl Into<USizeVec2>) {
        let point = point.into();
        let width = self.width();
        let height = self.height();

        if point.x < width && point.y < height {
            let index = point.y * width + point.x;
            self.buffer[index] = self.color;
        }
    }
}

// 2D Space
impl Context {
    pub fn ndc_to_screen(&self, ndc: Vec2) -> Vec2 {
        let width = self.width() as f32;
        let height = self.height() as f32;

        let x = (ndc.x + 1.0) / 2.0 * width;
        let y = (1.0 - ndc.y) / 2.0 * height;

        Vec2::new(x, y)
    }

    pub fn draw_point_2d(&mut self, point: impl Into<Vec2>) {
        let point = self.ndc_to_screen(point.into());
        self.draw_pixel(USizeVec2::new(point.x.round() as usize, point.y.round() as usize));
    }

    pub fn draw_line_2d(&mut self, from: impl Into<Vec2>, to: impl Into<Vec2>) {
        let from = self.ndc_to_screen(from.into());
        let to = self.ndc_to_screen(to.into());

        let dx = (to.x - from.x).abs();
        let dy = (to.y - from.y).abs();
        let dist = dx.max(dy).round() as usize;

        let steps = dist.max(1) as f32;

        for n in 0..=dist {
            let t = n as f32 / steps;
            let x = from.x + t * (to.x - from.x);
            let y = from.y + t * (to.y - from.y);

            if x >= 0.0 && x < self.width() as f32 && y >= 0.0 && y < self.height() as f32 {
                self.draw_pixel(USizeVec2::new(x.round() as usize, y.round() as usize));
            }
        }
    }
}

// 3D Space
impl Context {
    pub fn draw_point_3d(&mut self, point: impl Into<Vec3>, camera: &Camera) {
        let width = self.width();
        let height = self.height();
        let point_ndc = camera.project(point.into(), width, height);
        self.draw_point_2d(point_ndc);
    }

    pub fn draw_line_3d(&mut self, from: impl Into<Vec3>, to: impl Into<Vec3>, camera: &Camera) {
        let width = self.width();
        let height = self.height();

        let from_ndc = camera.project(from.into(), width, height);
        let to_ndc = camera.project(to.into(), width, height);

        self.draw_line_2d(from_ndc, to_ndc);
    }

    pub fn draw_grid_3d(
        &mut self,
        center: impl Into<Vec3>,
        size: impl Into<Vec2>,
        camera: &Camera,
    ) {
        let center = center.into();
        let size = size.into();
        let half_size = size / 2.0;

        let min_x = center.x - half_size.x;
        let max_x = center.x + half_size.x;
        let min_z = center.z - half_size.y;
        let max_z = center.z + half_size.y;

        for x in (min_x as isize)..=(max_x as isize) {
            let from = Vec3::new(x as f32, center.y, min_z);
            let to = Vec3::new(x as f32, center.y, max_z);
            self.draw_line_3d(from, to, camera);
        }

        for z in (min_z as isize)..=(max_z as isize) {
            let from = Vec3::new(min_x, center.y, z as f32);
            let to = Vec3::new(max_x, center.y, z as f32);
            self.draw_line_3d(from, to, camera);
        }
    }
}

pub enum Projection {
    Orthographic,
    Perspective { fov: f32 },
}

pub struct Camera {
    pub position: Vec3,
    pub projection: Projection,
}

impl Camera {
    pub fn project(&self, point: impl Into<Vec3>, width: usize, height: usize) -> Vec2 {
        let point = point.into();
        let width = width as f32;
        let height = height as f32;

        match self.projection {
            Projection::Orthographic => todo!(),
            Projection::Perspective { fov } => {
                let dir = point - self.position;
                let depth = dir.z;

                if depth < 0.0001 {
                    return Vec2::ZERO;
                }

                let scale = (fov.to_radians() / 2.0).tan() * depth;
                let aspect_ratio = width / height;

                let x = (dir.x / aspect_ratio) / scale;
                let y = dir.y / scale;

                Vec2 { x, y }
            }
        }
    }
}
