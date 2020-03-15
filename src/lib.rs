use minifb::{Window, WindowOptions};

pub struct WindowBuffer {
    buffer: Vec<u32>,
    width: usize,
    height: usize,
}

impl WindowBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            buffer: vec![0; width * height],
            width,
            height,
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        let l = y * self.width + x;
        assert!(l <= self.width * self.height - 2);
        self.buffer[l] = color;
    }

    pub fn draw(&mut self) {
        let mut window = Window::new(
            "Mandelbrot Set",
            self.width,
            self.height,
            WindowOptions::default(),
        )
        .expect("unable to set up window");

        while window.is_open() {
            window
                .update_with_buffer(&self.buffer)
                .expect("unable to update window");
        }
    }
}
