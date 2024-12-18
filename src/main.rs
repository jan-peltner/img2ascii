use clap::Parser;
use image::{imageops, ImageReader};
use img2ascii::ascii::AsciiArt;

const MAX_WIDTH: u32 = 200;
const MAX_HEIGHT: u32 = 50;

#[derive(Parser)]
struct Cli {
    /// path to image
    file_path: String,
}

fn main() {
    let cli = Cli::parse();
    let img = ImageReader::open(&cli.file_path).expect("Could not read image");
    let mut img = img.decode().expect("Could not decode image");
    if img.width() > MAX_WIDTH || img.height() > MAX_HEIGHT {
        img = img.resize(MAX_WIDTH, MAX_HEIGHT, imageops::FilterType::Nearest);
    }
    let mut art = AsciiArt::with_width(img.width());
    art.draw(&img);
    art.print();
}
