use mandelbrot_set::*;

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

    write_png(&buffer.buffer());
}

fn write_png(buffer: &[u8]) {
    use png::{BitDepth, ColorType, Encoder};
    use std::fs::File;
    use std::io::BufWriter;
    use std::path::Path;

    let path = Path::new(r"out.png");
    let file = File::create(path).unwrap();
    let w = BufWriter::new(file);

    let mut encoder = Encoder::new(w, WIDTH as u32, HEIGHT as u32);
    encoder.set_color(ColorType::RGBA);
    encoder.set_depth(BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(buffer).unwrap();
}
