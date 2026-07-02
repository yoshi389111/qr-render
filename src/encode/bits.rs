use crate::BinaryImage;

pub fn get_bits_from_image(
    image: &impl BinaryImage,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
) -> u64 {
    let mut bits: u64 = 0;
    for dy in 0..h {
        for dx in 0..w {
            if y + dy < image.height() && x + dx < image.width() && image.get(x + dx, y + dy) {
                let bit_index = dy * w + dx;
                bits |= 1 << bit_index;
            }
        }
    }
    bits
}
