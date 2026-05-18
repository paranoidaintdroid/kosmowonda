use kosmo_math::Vec2;
use minifb::{Window, WindowOptions};

pub struct Renderer {
    window: Window,
    buffer: Vec<u32>,
    width: usize,
    height: usize,
}

impl Renderer {
    pub fn new(title: &str, width: usize, height: usize) -> Self {
        let window = Window::new(title, width, height, WindowOptions::default())
            .expect("failed to create window");

        let buffer = vec![0u32; width * height];

        Self {
            window,
            buffer,
            width,
            height,
        }
    }

    pub fn clear(&mut self, color: u32) {
        self.buffer.iter_mut().for_each(|pixel| *pixel = color);
    }

    pub fn draw_dot(&mut self, x: usize, y: usize, color: u32) {
        if x >= self.width || y >= self.height {
            return;
        }

        let index = y * self.width + x;

        self.buffer[index] = color;
    }

    pub fn present(&mut self) {
        self.window
            .update_with_buffer(&self.buffer, self.width, self.height)
            .expect("failed to present frame");
    }

    pub fn world_to_screen(&self, pos: Vec2, world_scale: f64) -> (usize, usize) {
        let center_x = self.width as f64 / 2.0;

        let center_y = self.height as f64 / 2.0;

        let screen_x = center_x + pos.x * world_scale;

        let screen_y = center_y - pos.y * world_scale;

        let clamped_x = screen_x.clamp(0.0, (self.width - 1) as f64) as usize;

        let clamped_y = screen_y.clamp(0.0, (self.height - 1) as f64) as usize;

        (clamped_x, clamped_y)
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }
}
