use crate::prelude::*;

pub struct Context {
    window: Window,
    buffer: Vec<u32>,

    camera: Option<Object<Camera>>,

    translation_stack: Vec<Vec3>,
    rotation_stack: Vec<Quat>,
    color_stack: Vec<u32>,
}

impl Context {
    pub fn new(width: usize, height: usize) -> crate::window::Result<Self> {
        let window = Window::new("MAAK", width, height, WindowOptions::default())?;

        let mut this = Self {
            window,
            buffer: vec![0u32; width * height],

            camera: None,

            translation_stack: Vec::new(),
            rotation_stack: Vec::new(),
            color_stack: Vec::new(),
        };

        this.camera = Some(this.spawn(|_| Camera::new(width, height)));

        Ok(this)
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

    pub fn clear_buffer(&mut self) {
        self.buffer.fill(0x000000);
    }

    pub fn update_with_buffer(&mut self) -> crate::window::Result<()> {
        let (width, height) = self.window().get_size();
        self.window.update_with_buffer(&self.buffer, width, height)
    }

    pub fn camera(&self) -> &Object<Camera> {
        self.camera.as_ref().expect("camera should be initialized")
    }

    pub fn camera_mut(&mut self) -> &mut Object<Camera> {
        self.camera.as_mut().expect("camera should be initialized")
    }

    pub fn push_translation(&mut self, translation: Vec3) {
        let current = self.translation();
        self.translation_stack.push(current + translation);
    }

    pub fn pop_translation(&mut self) {
        self.translation_stack.pop();
    }

    pub fn translation(&self) -> Vec3 {
        self.translation_stack.last().copied().unwrap_or(Vec3::ZERO)
    }

    pub fn push_rotation(&mut self, rotation: Quat) {
        let current = self.rotation();
        self.rotation_stack.push(rotation * current);
    }

    pub fn pop_rotation(&mut self) {
        self.rotation_stack.pop();
    }

    pub fn rotation(&self) -> Quat {
        self.rotation_stack.last().copied().unwrap_or(Quat::IDENTITY)
    }

    pub fn push_color(&mut self, color: u32) {
        self.color_stack.push(color);
    }

    pub fn pop_color(&mut self) -> u32 {
        self.color_stack.pop().unwrap_or(self.color())
    }

    pub fn set_color(&mut self, color: u32) {
        self.color_stack.clear();
        self.push_color(color);
    }

    pub fn color(&self) -> u32 {
        match self.color_stack.last() {
            Some(color) => *color,
            None => 0xffffff,
        }
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
            self.buffer[index] = self.color();
        }
    }
}

// 2D Space
impl Context {
    pub fn ndc_to_screen(&self, ndc: Vec2) -> Vec2 {
        let w = self.width() as f32;
        let h = self.height() as f32;
        let x = (ndc.x + 1.0) / 2.0 * w;
        let y = (1.0 - ndc.y) / 2.0 * h;
        Vec2::new(x, y)
    }

    pub fn draw_point_2d(&mut self, point: impl Into<Vec2>) {
        let point = self.ndc_to_screen(point.into());
        self.draw_pixel(USizeVec2::new(point.x.round() as usize, point.y.round() as usize));
    }

    pub fn draw_line_2d(&mut self, from: impl Into<Vec2>, to: impl Into<Vec2>) {
        // https://en.wikipedia.org/wiki/Digital_differential_analyzer_(graphics_algorithm)

        let from = self.ndc_to_screen(from.into());
        let to = self.ndc_to_screen(to.into());
        let dx = to.x - from.x;
        let dy = to.y - from.y;
        let step = if dx.abs() >= dy.abs() { dx.abs() } else { dy.abs() };

        let dx = dx / step;
        let dy = dy / step;
        let mut x = from.x;
        let mut y = from.y;
        let mut i = 0;

        while i as f32 <= step {
            self.draw_pixel((x.round() as usize, y.round() as usize));
            x = x + dx;
            y = y + dy;
            i = i + 1;
        }
    }
}

// 3D Space
impl Context {
    pub fn draw_point_3d(&mut self, point: impl Into<Vec3>) {
        let point = self.transformed(point.into());

        if let Some(ndc) = self.camera().project(point) {
            self.draw_point_2d(ndc);
        }
    }

    pub fn draw_line_3d(&mut self, from: impl Into<Vec3>, to: impl Into<Vec3>) {
        let v0 = self.camera().to_camera_space(self.transformed(from.into()));
        let v1 = self.camera().to_camera_space(self.transformed(to.into()));

        if let Some((clipped_v0, clipped_v1)) = self.camera().clip_line_camera_space(v0, v1) {
            let ndc0 = self.camera().project_camera_space(clipped_v0);
            let ndc1 = self.camera().project_camera_space(clipped_v1);

            if let (Some(ndc0), Some(ndc1)) = (ndc0, ndc1) {
                self.draw_line_2d(ndc0, ndc1);
            }
        }
    }

    fn transformed(&self, point: Vec3) -> Vec3 {
        self.translation() + self.rotation() * point
    }
}
