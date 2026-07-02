use clap::Parser;
use qr_render::QrCodeBitmap;

#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    /// The data to encode in the QR code
    data: String,

    /// The size of the quiet zone around the QR code
    #[arg(short, long, default_value_t = 2)]
    quiet_zone: usize,

    /// The style of the QR code
    #[arg(short, long, default_value = "half", value_parser = ["braille", "half", "octant", "quadrant", "separated-quadrant", "separated-sextant", "sextant"])]
    style: String,
}

fn main() {
    let args = Args::parse();

    let qr_bitmap = QrCodeBitmap::new(args.data.as_bytes(), args.quiet_zone);

    let output = match args.style.as_str() {
        "braille" => qr_render::Braille::render(&qr_bitmap),
        "half" => qr_render::Half::render(&qr_bitmap),
        "octant" => qr_render::Octant::render(&qr_bitmap),
        "quadrant" => qr_render::Quadrant::render(&qr_bitmap),
        "separated-quadrant" => qr_render::SeparatedQuadrant::render(&qr_bitmap),
        "separated-sextant" => qr_render::SeparatedSextant::render(&qr_bitmap),
        "sextant" => qr_render::Sextant::render(&qr_bitmap),
        _ => qr_render::Half::render(&qr_bitmap), // Default to half
    };
    print!("{output}");
}
