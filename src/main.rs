use mandelbrot_set::*;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

const WIDTH: usize = 1400;
const HEIGHT: usize = 800;

fn main() {
    let mut buffer = Buffer::new(WIDTH, HEIGHT);

    let mandelbrot_set = MandelbrotSet::default();
    mandelbrot_set.generate(|x, y, color| {
        buffer.set_pixel(
            mandelbrot_set.scale(x, WIDTH as f64 - 1.0) as usize,
            mandelbrot_set.scale(y, HEIGHT as f64 - 1.0) as usize,
            color,
        );
    });

    let path = Path::new(r"out.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, WIDTH as u32, HEIGHT as u32);
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(buffer.buffer()).unwrap();
}
