use super::get_bits_from_image;
use crate::BinaryImage;

pub struct SeparatedQuadrant;

impl SeparatedQuadrant {
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
    '\u{0020}',
    '\u{1CC21}',
    '\u{1CC22}',
    '\u{1CC23}',
    '\u{1CC24}',
    '\u{1CC25}',
    '\u{1CC26}',
    '\u{1CC27}',
    '\u{1CC28}',
    '\u{1CC29}',
    '\u{1CC2A}',
    '\u{1CC2B}',
    '\u{1CC2C}',
    '\u{1CC2D}',
    '\u{1CC2E}',
    '\u{1CC2F}',
];
