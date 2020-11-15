use super::Color;

pub struct Canvas {
    width: i32,
    height: i32,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: i32, height: i32) -> Canvas {
        Canvas {
            width,
            height,
            pixels: vec![Color::new(0.0, 0.0, 0.0); (width * height) as usize],
        }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn write_pixel(&mut self, x: i32, y: i32, col: Color) {
        let index = self.coord_to_index(x, y);
        self.pixels[index] = col;
    }

    pub fn pixel_at(&self, x: i32, y: i32) -> Color {
        let index = self.coord_to_index(x, y);
        if index >= (self.width * self.height) as usize {
            panic!("Hello");
        }
        self.pixels[index]
    }

    fn coord_to_index(&self, x: i32, y: i32) -> usize {
        (x + y * self.width()) as usize
    }

    pub fn get_save_buffer(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.reserve((self.width() * self.height() * 4) as usize);

        // Convert each color into [u8; 4]
        for pixel in self.pixels.iter() {
            let r = (pixel.r().min(1.0).max(0.0) * 255.0) as u8;
            let g = (pixel.g().min(1.0).max(0.0) * 255.0) as u8;
            let b = (pixel.b().min(1.0).max(0.0) * 255.0) as u8;
            result.push(r);
            result.push(g);
            result.push(b);
            result.push(255);
        }

        result
    }

    pub fn downsample(&self, sample_width: i32, sample_height: i32) -> Canvas {
        let mut new_canvas = Canvas::new(sample_width, sample_height);
        let (half_pixel_width, half_pixel_height) = (
            1.0 / sample_width as f64 / 2.0,
            1.0 / sample_height as f64 / 2.0,
        );

        let (width, height) = (self.width / sample_width, self.height / sample_height);

        for y in 0..sample_height {
            for x in 0..sample_width {
                let pixel_color = self.sample_at(
                    x as f64 * (half_pixel_width * 2.0) + half_pixel_width,
                    y as f64 * (half_pixel_height * 2.0) + half_pixel_height,
                    width,
                    height,
                );
                new_canvas.write_pixel(x, y, pixel_color);
            }
        }

        new_canvas
    }

    pub fn sample_at(
        &self,
        coord_x: f64,
        coord_y: f64,
        sample_width: i32,
        sample_height: i32,
    ) -> Color {
        // Convert sample size to relative canvas size
        let (half_rel_sample_width, half_rel_sample_height) = (
            sample_width as f64 / self.width as f64 / 4.0,
            sample_height as f64 / self.height as f64 / 4.0,
        );

        // Find the top left pixel
        let top_left = self.float_pixel_at(
            coord_x - half_rel_sample_width,
            coord_y - half_rel_sample_height,
        );

        // Find the top right pixel
        let top_right = self.float_pixel_at(
            coord_x + half_rel_sample_width,
            coord_y - half_rel_sample_height,
        );

        // Find the bottom left pixel
        let bottom_left = self.float_pixel_at(
            coord_x - half_rel_sample_width,
            coord_y + half_rel_sample_height,
        );

        // Find the bottom right pixel
        let bottom_right = self.float_pixel_at(
            coord_x + half_rel_sample_width,
            coord_y + half_rel_sample_height,
        );

        // Average the pixels together
        let sum: Color = top_left + top_right + bottom_left + bottom_right;
        Color::new(sum.r() / 4.0, sum.g() / 4.0, sum.b() / 4.0)
    }

    fn float_pixel_at(&self, coord_x: f64, coord_y: f64) -> Color {
        let (actual_x, actual_y) = (self.width as f64 * coord_x, self.height as f64 * coord_y);
        self.pixel_at(actual_x as i32, actual_y as i32)
    }
}
