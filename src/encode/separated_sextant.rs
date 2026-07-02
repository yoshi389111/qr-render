use super::get_bits_from_image;
use crate::BinaryImage;

pub struct SeparatedSextant;

impl SeparatedSextant {
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
    '\u{1CE51}',
    '\u{1CE52}',
    '\u{1CE53}',
    '\u{1CE54}',
    '\u{1CE55}',
    '\u{1CE56}',
    '\u{1CE57}',
    '\u{1CE58}',
    '\u{1CE59}',
    '\u{1CE5A}',
    '\u{1CE5B}',
    '\u{1CE5C}',
    '\u{1CE5D}',
    '\u{1CE5E}',
    '\u{1CE5F}',
    '\u{1CE60}',
    '\u{1CE61}',
    '\u{1CE62}',
    '\u{1CE63}',
    '\u{1CE64}',
    '\u{1CE65}',
    '\u{1CE66}',
    '\u{1CE67}',
    '\u{1CE68}',
    '\u{1CE69}',
    '\u{1CE6A}',
    '\u{1CE6B}',
    '\u{1CE6C}',
    '\u{1CE6D}',
    '\u{1CE6E}',
    '\u{1CE6F}',
    '\u{1CE70}',
    '\u{1CE71}',
    '\u{1CE72}',
    '\u{1CE73}',
    '\u{1CE74}',
    '\u{1CE75}',
    '\u{1CE76}',
    '\u{1CE77}',
    '\u{1CE78}',
    '\u{1CE79}',
    '\u{1CE7A}',
    '\u{1CE7B}',
    '\u{1CE7C}',
    '\u{1CE7D}',
    '\u{1CE7E}',
    '\u{1CE7F}',
    '\u{1CE80}',
    '\u{1CE81}',
    '\u{1CE82}',
    '\u{1CE83}',
    '\u{1CE84}',
    '\u{1CE85}',
    '\u{1CE86}',
    '\u{1CE87}',
    '\u{1CE88}',
    '\u{1CE89}',
    '\u{1CE8A}',
    '\u{1CE8B}',
    '\u{1CE8C}',
    '\u{1CE8D}',
    '\u{1CE8E}',
    '\u{1CE8F}',
];
