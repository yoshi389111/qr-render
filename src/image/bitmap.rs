use super::BinaryImage;

/// A monochrome bitmap image.
pub struct Bitmap {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u64>,
}

impl Bitmap {
    pub fn new(width: usize, height: usize) -> Self {
        let width_words = width.div_ceil(64);
        let num_words = width_words * height;
        Bitmap {
            width,
            height,
            pixels: vec![0; num_words],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        if x < self.width && y < self.height {
            let width_words = self.width.div_ceil(64);
            let index = y * width_words + x / 64;
            let bit_index = x % 64;
            ((self.pixels[index] >> bit_index) & 1) != 0
        } else {
            false
        }
    }

    pub fn set(&mut self, x: usize, y: usize, value: bool) {
        if x < self.width && y < self.height {
            let width_words = self.width.div_ceil(64);
            let index = y * width_words + x / 64;
            let bit_index = x % 64;
            if value {
                self.pixels[index] |= 1 << bit_index;
            } else {
                self.pixels[index] &= !(1 << bit_index);
            }
        }
    }
}

impl BinaryImage for Bitmap {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn get(&self, x: usize, y: usize) -> bool {
        Bitmap::get(self, x, y)
    }
}
