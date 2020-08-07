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
}
