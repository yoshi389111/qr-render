use crate::BinaryImage;
use qrcode::QrCode;

pub struct QrCodeBitmap {
    pub code: QrCode,
    pub quiet_zone: usize,
}

impl QrCodeBitmap {
    pub fn new(data: &[u8], quiet_zone: usize) -> Result<Self, qrcode::types::QrError> {
        assert!(quiet_zone <= 4, "Quiet zone must be between 0 and 4");

        let code = QrCode::new(data)?;
        Ok(Self { code, quiet_zone })
    }
}

impl BinaryImage for QrCodeBitmap {
    fn width(&self) -> usize {
        self.code.width() + 2 * self.quiet_zone
    }

    fn height(&self) -> usize {
        self.code.width() + 2 * self.quiet_zone
    }

    fn get(&self, x: usize, y: usize) -> bool {
        if x < self.quiet_zone
            || y < self.quiet_zone
            || x >= self.width() - self.quiet_zone
            || y >= self.height() - self.quiet_zone
        {
            false
        } else {
            let qrcode_x = x - self.quiet_zone;
            let qrcode_y = y - self.quiet_zone;
            self.code[(qrcode_x, qrcode_y)] == qrcode::Color::Dark
        }
    }
}
