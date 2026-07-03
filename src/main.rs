use clap::{Parser, value_parser};
use qr_render::QrCodeBitmap;

#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    /// The data to encode in the QR code
    data: String,

    /// The size of the quiet zone around the QR code
    #[arg(short, long, default_value_t = 2, value_parser = value_parser!(u8).range(0..=4))]
    quiet_zone: u8,

    /// The style of the QR code
    #[arg(short, long, default_value = "half", value_parser = ["braille", "half", "octant", "quadrant", "separated-quadrant", "separated-sextant", "sextant", "svg"])]
    style: String,

    /// The output file for the QR code
    #[arg(short, long)]
    output: Option<String>,
}

fn main() {
    let args = Args::parse();

    let data = args.data.as_bytes();
    let quiet_zone = args.quiet_zone as usize;
    let qr_bitmap = QrCodeBitmap::new(data, quiet_zone).unwrap_or_else(|e| {
        eprintln!("Failed to create QR code: {}", e);
        std::process::exit(1);
    });

    let output_data = match args.style.as_str() {
        "braille" => qr_render::Braille::render(&qr_bitmap),
        "half" => qr_render::Half::render(&qr_bitmap),
        "octant" => qr_render::Octant::render(&qr_bitmap),
        "quadrant" => qr_render::Quadrant::render(&qr_bitmap),
        "separated-quadrant" => qr_render::SeparatedQuadrant::render(&qr_bitmap),
        "separated-sextant" => qr_render::SeparatedSextant::render(&qr_bitmap),
        "sextant" => qr_render::Sextant::render(&qr_bitmap),
        "svg" => qr_render::SvgEncoder::render(&qr_bitmap),
        _ => qr_render::Half::render(&qr_bitmap), // Default to half
    };
    if let Some(output) = args.output {
        std::fs::write(output, output_data).unwrap_or_else(|e| {
            eprintln!("Failed to write output file: {}", e);
            std::process::exit(1);
        });
    } else {
        print!("{output_data}");
    }
}
