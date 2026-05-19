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

    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }

    pub fn clear(&mut self, color: u32) {
        self.buffer.iter_mut().for_each(|pixel| *pixel = color);
    }

    pub fn fade(&mut self, amount: u8) {
        for pixel in self.buffer.iter_mut() {
            let r = ((*pixel >> 16) & 0xFF) as u8;

            let g = ((*pixel >> 8) & 0xFF) as u8;

            let b = (*pixel & 0xFF) as u8;

            let r = r.saturating_sub(amount);

            let g = g.saturating_sub(amount);

            let b = b.saturating_sub(amount);

            *pixel = ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
        }
    }

    pub fn draw_dot(&mut self, x: usize, y: usize, color: u32) {
        if x >= self.width || y >= self.height {
            return;
        }

        let index = y * self.width + x;

        self.buffer[index] = color;
    }

    pub fn draw_circle(&mut self, cx: isize, cy: isize, radius: isize, color: u32) {
        for y in (cy - radius)..=(cy + radius) {
            for x in (cx - radius)..=(cx + radius) {
                let dx = x - cx;
                let dy = y - cy;

                if dx * dx + dy * dy <= radius * radius {
                    if x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height {
                        let index = y as usize * self.width + x as usize;

                        self.buffer[index] = color;
                    }
                }
            }
        }
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
}
