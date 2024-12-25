use clap::Parser;
use image::{imageops, ImageReader};
use img2ascii::ascii::{brighten, AsciiArt};

const MAX_WIDTH: u32 = 300;
const MAX_HEIGHT: u32 = 50;

#[derive(Parser)]
struct Cli {
    /// path to image
    file_path: String,
}

fn main() {
    let cli = Cli::parse();
    let img = ImageReader::open(&cli.file_path)
        .expect("Could not read image")
        .decode()
        .expect("Could not decode image");
    let mut processed_img = brighten(img);

    if processed_img.width() > MAX_WIDTH || processed_img.height() > MAX_HEIGHT {
        processed_img = processed_img.resize(MAX_WIDTH, MAX_HEIGHT, imageops::FilterType::Nearest);
    }
    let mut art = AsciiArt::with_width(processed_img.width());
    art.construct_pixels_from_img(&processed_img);
    art.draw();
}
