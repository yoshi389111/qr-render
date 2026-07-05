use clap::{Parser, value_parser};
use qr_render::QrCodeBitmap;

#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    /// The data to encode in the QR code
    data: String,

    /// The size of the quiet zone around the QR code [range: 0-4]
    #[arg(short, long, default_value_t = 2, value_parser = value_parser!(u8).range(0..=4))]
    quiet_zone: u8,

    /// The error correction level of the QR code
    #[arg(short, long, default_value = "M", value_enum, ignore_case = true)]
    error_level: ErrorCorrectionLevelArg,

    /// The style of the QR code
    #[arg(short, long, default_value = "half", value_enum, ignore_case = true)]
    style: OutputStyleArg,

    /// The output file for the QR code
    #[arg(short, long)]
    output: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    run(args)
}

fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let data = args.data.as_bytes();
    let quiet_zone = args.quiet_zone as usize;

    let error_level = match args.error_level {
        ErrorCorrectionLevelArg::L => qr_render::ErrorLevel::Low,
        ErrorCorrectionLevelArg::M => qr_render::ErrorLevel::Medium,
        ErrorCorrectionLevelArg::Q => qr_render::ErrorLevel::Quartile,
        ErrorCorrectionLevelArg::H => qr_render::ErrorLevel::High,
    };

    let qr_bitmap = QrCodeBitmap::new(data, error_level, quiet_zone)
        .map_err(|e| format!("Failed to create QR code: {}", e))?;

    let output_data = match args.style {
        OutputStyleArg::Braille => qr_render::Braille::render(&qr_bitmap),
        OutputStyleArg::Half => qr_render::Half::render(&qr_bitmap),
        OutputStyleArg::Octant => qr_render::Octant::render(&qr_bitmap),
        OutputStyleArg::Quadrant => qr_render::Quadrant::render(&qr_bitmap),
        OutputStyleArg::SeparatedQuadrant => qr_render::SeparatedQuadrant::render(&qr_bitmap),
        OutputStyleArg::SeparatedSextant => qr_render::SeparatedSextant::render(&qr_bitmap),
        OutputStyleArg::Sextant => qr_render::Sextant::render(&qr_bitmap),
        OutputStyleArg::Svg => qr_render::SvgEncoder::render(&qr_bitmap),
    };
    if let Some(output) = args.output {
        std::fs::write(&output, output_data)
            .map_err(|e| format!("Failed to write output file `{}`: {}", output, e))?;
    } else {
        print!("{output_data}");
    }
    Ok(())
}

/// Error correction level for the QR code
#[derive(Debug, Clone, PartialEq, Eq, clap::ValueEnum)]
enum ErrorCorrectionLevelArg {
    #[value(name = "L")]
    L,
    #[value(name = "M")]
    M,
    #[value(name = "Q")]
    Q,
    #[value(name = "H")]
    H,
}

#[derive(Debug, Clone, PartialEq, Eq, clap::ValueEnum)]
enum OutputStyleArg {
    Braille,
    Half,
    Octant,
    Quadrant,
    SeparatedQuadrant,
    SeparatedSextant,
    Sextant,
    Svg,
}
