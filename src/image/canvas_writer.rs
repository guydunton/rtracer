use super::Canvas;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

pub fn save_canvas(canvas: Canvas, path: String) -> Result<(), png::EncodingError> {
    // Create file & writer
    let out_path = Path::new(&path);
    let file = File::create(out_path)?;
    let buf_writer = BufWriter::new(file);
    // Create png writer
    let mut encoder = png::Encoder::new(buf_writer, canvas.width() as u32, canvas.height() as u32);
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;

    // Write image data
    let image_data = canvas.get_save_buffer();
    writer.write_image_data(&image_data)?;

    Ok(())
}
