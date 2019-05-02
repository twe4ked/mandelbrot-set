use mandelbrot_set::WindowBuffer;
use num_complex::Complex;

const WIDTH: usize = 1400;
const HEIGHT: usize = 800;
const EPSILON: f64 = 0.0001;
const MAX_ITERATIONS: u8 = 20;
const MANDELBROT_RANGE: (f64, f64) = (-2.0, 2.0);

fn main() {
    let mut window_buffer = WindowBuffer::new(WIDTH, HEIGHT);

    let mut x: f64 = MANDELBROT_RANGE.0;
    let mut y: f64 = MANDELBROT_RANGE.0;
    let mut c: Complex<f64>;
    let mut z: Complex<f64>;
    let mut iterations;

    while x <= MANDELBROT_RANGE.1 {
        while y <= MANDELBROT_RANGE.1 {
            iterations = 0;

            c = Complex::new(x, y);
            z = Complex::new(0.0, 0.0);

            while z.norm() < MANDELBROT_RANGE.1 && iterations < MAX_ITERATIONS {
                z = z * z + c;
                iterations += 1;
            }

            let mut color: u32 = 0;
            if iterations != MAX_ITERATIONS {
                color = (50 + iterations * 10 % 255).into();
            }

            window_buffer.set_pixel(
                scale(x, WIDTH as f64 - 1.0),
                scale(y, HEIGHT as f64 - 1.0),
                color,
            );

            y += EPSILON
        }
        y = MANDELBROT_RANGE.0;

        x += EPSILON
    }

    window_buffer.draw();
}

fn scale(input: f64, y2: f64) -> usize {
    let x1: f64 = MANDELBROT_RANGE.0;
    let x2: f64 = MANDELBROT_RANGE.1;
    let y1: f64 = 0.0;

    ((((input - x1) * (y2 - y1)) / (x2 - x1)) + y1) as usize
}
