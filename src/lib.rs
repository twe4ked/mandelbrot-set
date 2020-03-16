mod buffer;

pub use buffer::Buffer;
use num_complex::Complex;

pub struct MandelbrotSet {
    epsilon: f64,
    max_iterations: u32,
    range: (f64, f64),
}

impl Default for MandelbrotSet {
    fn default() -> Self {
        Self {
            epsilon: 0.0001,
            max_iterations: 20,
            range: (-2.0, 2.0),
        }
    }
}

impl MandelbrotSet {
    pub fn generate<F>(&self, mut callback: F)
    where
        F: FnMut(f64, f64, u32),
    {
        let mut x: f64 = self.range.0;
        let mut y: f64 = self.range.0;
        let mut c: Complex<f64>;
        let mut z: Complex<f64>;
        let mut iterations: u32;

        while x <= self.range.1 {
            while y <= self.range.1 {
                iterations = 0;

                c = Complex::new(x, y);
                z = Complex::new(0.0, 0.0);

                while z.norm() < self.range.1 && iterations < self.max_iterations {
                    z = z * z + c;
                    iterations += 1;
                }

                let color = if iterations != self.max_iterations {
                    50 + iterations * 10 % 255
                } else {
                    0
                };

                callback(x, y, color);

                y += self.epsilon
            }
            y = self.range.0;

            x += self.epsilon
        }
    }

    pub fn scale(&self, input: f64, y2: f64) -> f64 {
        let x1: f64 = self.range.0;
        let x2: f64 = self.range.1;
        let y1: f64 = 0.0;
        (((input - x1) * (y2 - y1)) / (x2 - x1)) + y1
    }
}
