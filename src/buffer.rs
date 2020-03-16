pub struct Buffer {
    buffer: Vec<u8>,
    width: usize,
    height: usize,
}

impl Buffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            // * 4 because each color is represented by an RGBA sequence.
            buffer: vec![0; width * height * 4],
            width,
            height,
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        let l = (y * self.width + x) * 4;
        assert!(l <= self.width * self.height * 4 - 2);

        // Ignore the top bits (0RGB -> RGBA)
        // let alpha = color >> 24 & 0xff;
        let red = color >> 16 & 0xff;
        let green = color >> 8 & 0xff;
        let blue = color >> 0 & 0xff;

        self.buffer[l + 0] = red as u8;
        self.buffer[l + 1] = green as u8;
        self.buffer[l + 2] = blue as u8;
        self.buffer[l + 3] = 255;
    }

    pub fn buffer(&self) -> &[u8] {
        &self.buffer
    }
}
