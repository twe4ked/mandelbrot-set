use mandelbrot_set::*;

const WIDTH: usize = 1400;
const HEIGHT: usize = 800;
const EPSILON: f64 = 0.0001;
const MAX_ITERATIONS: u32 = 20;
const MANDELBROT_RANGE: (f64, f64) = (-2.0, 2.0);

fn main() {
    let mut window_buffer = WindowBuffer::new(WIDTH, HEIGHT);

    generate(EPSILON, MAX_ITERATIONS, MANDELBROT_RANGE, |x, y, color| {
        window_buffer.set_pixel(
            scale(x, WIDTH as f64 - 1.0),
            scale(y, HEIGHT as f64 - 1.0),
            color,
        );
    });

    window_buffer.draw();
}

fn scale(input: f64, y2: f64) -> usize {
    let x1: f64 = MANDELBROT_RANGE.0;
    let x2: f64 = MANDELBROT_RANGE.1;
    let y1: f64 = 0.0;

    ((((input - x1) * (y2 - y1)) / (x2 - x1)) + y1) as usize
}
