use super::get_bits_from_image;
use crate::BinaryImage;

pub struct Sextant;

impl Sextant {
    pub const WIDTH: usize = 2;
    pub const HEIGHT: usize = 3;

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
    '\u{1FB00}',
    '\u{1FB01}',
    '\u{1FB02}',
    '\u{1FB03}',
    '\u{1FB04}',
    '\u{1FB05}',
    '\u{1FB06}',
    '\u{1FB07}',
    '\u{1FB08}',
    '\u{1FB09}',
    '\u{1FB0A}',
    '\u{1FB0B}',
    '\u{1FB0C}',
    '\u{1FB0D}',
    '\u{1FB0E}',
    '\u{1FB0F}',
    '\u{1FB10}',
    '\u{1FB11}',
    '\u{1FB12}',
    '\u{1FB13}',
    '\u{258C}',
    '\u{1FB14}',
    '\u{1FB15}',
    '\u{1FB16}',
    '\u{1FB17}',
    '\u{1FB18}',
    '\u{1FB19}',
    '\u{1FB1A}',
    '\u{1FB1B}',
    '\u{1FB1C}',
    '\u{1FB1D}',
    '\u{1FB1E}',
    '\u{1FB1F}',
    '\u{1FB20}',
    '\u{1FB21}',
    '\u{1FB22}',
    '\u{1FB23}',
    '\u{1FB24}',
    '\u{1FB25}',
    '\u{1FB26}',
    '\u{1FB27}',
    '\u{2590}',
    '\u{1FB28}',
    '\u{1FB29}',
    '\u{1FB2A}',
    '\u{1FB2B}',
    '\u{1FB2C}',
    '\u{1FB2D}',
    '\u{1FB2E}',
    '\u{1FB2F}',
    '\u{1FB30}',
    '\u{1FB31}',
    '\u{1FB32}',
    '\u{1FB33}',
    '\u{1FB34}',
    '\u{1FB35}',
    '\u{1FB36}',
    '\u{1FB37}',
    '\u{1FB38}',
    '\u{1FB39}',
    '\u{1FB3A}',
    '\u{1FB3B}',
    '\u{2588}',
];
