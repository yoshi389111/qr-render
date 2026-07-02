use super::get_bits_from_image;
use crate::BinaryImage;

pub struct Quadrant;

impl Quadrant {
    pub const WIDTH: usize = 2;
    pub const HEIGHT: usize = 2;

    pub fn from_bits(bits: u64) -> char {
        let index = bits as usize;
        if index < LOOKUP_TABLE.len() {
            LOOKUP_TABLE[index]
        } else {
            LOOKUP_TABLE[0]
        }
    }

    pub fn write_to<W: std::fmt::Write>(out: &mut W, image: &impl BinaryImage) -> std::fmt::Result {
        let width = image.width();
        let height = image.height();

        for y in (0..height).step_by(Self::HEIGHT) {
            for x in (0..width).step_by(Self::WIDTH) {
                let bits = get_bits_from_image(image, x, y, Self::WIDTH, Self::HEIGHT);
                out.write_char(Self::from_bits(bits))?;
            }
            out.write_char('\n')?;
        }
        Ok(())
    }

    pub fn render(image: &impl BinaryImage) -> String {
        let mut output = String::new();
        Self::write_to(&mut output, image).unwrap();
        output
    }
}

static LOOKUP_TABLE: &[char] = &[
    '\u{0020}', '\u{2598}', '\u{259D}', '\u{2580}', '\u{2596}', '\u{258C}', '\u{259E}', '\u{259B}',
    '\u{2597}', '\u{259A}', '\u{2590}', '\u{259C}', '\u{2584}', '\u{2599}', '\u{259F}', '\u{2588}',
];
