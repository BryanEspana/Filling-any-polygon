use crate::colors::Color;

pub struct FrameBuffer {
    width: usize,
    height: usize,
    pub buffer: Vec<Color>,
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        let black = Color::black();
        Self {
            width,
            height,
            buffer: vec![black; width * height],
        }
    }

    pub fn draw_pixel(&mut self, x: isize, y: isize, color: Color) {
        if x >= 0 && y >= 0 && x < self.width as isize && y < self.height as isize {
            let index = (x as usize) + (y as usize) * self.width;
            self.buffer[index] = color;
        }
    }

    pub fn save(&self, path: &str) {
        let mut imgbuf = image::ImageBuffer::new(self.width as u32, self.height as u32);
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let index = x as usize + y as usize * self.width;
            *pixel = image::Rgb([self.buffer[index].red, self.buffer[index].green, self.buffer[index].blue]);
        }
        imgbuf.save(path).unwrap();
    }
}
