use num_complex::Complex;

pub struct WindowBuffer {
    buffer: Vec<u8>,
    width: usize,
    height: usize,
}

impl WindowBuffer {
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

pub fn generate<F>(epsilon: f64, max_iterations: u32, mandelbrot_range: (f64, f64), mut f: F)
where
    F: FnMut(f64, f64, u32),
{
    let mut x: f64 = mandelbrot_range.0;
    let mut y: f64 = mandelbrot_range.0;
    let mut c: Complex<f64>;
    let mut z: Complex<f64>;
    let mut iterations: u32;

    while x <= mandelbrot_range.1 {
        while y <= mandelbrot_range.1 {
            iterations = 0;

            c = Complex::new(x, y);
            z = Complex::new(0.0, 0.0);

            while z.norm() < mandelbrot_range.1 && iterations < max_iterations {
                z = z * z + c;
                iterations += 1;
            }

            let color = if iterations != max_iterations {
                50 + iterations * 10 % 255
            } else {
                0
            };

            f(x, y, color);

            y += epsilon
        }
        y = mandelbrot_range.0;

        x += epsilon
    }
}
