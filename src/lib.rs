mod buffer;

pub use buffer::Buffer;
use num_complex::Complex;

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
