use image::{imageops, ImageReader};
use img2ascii::ascii::AsciiArt;

const MAX_WIDTH: u32 = 200;
const MAX_HEIGHT: u32 = 50;

fn main() {
    let img = ImageReader::open("assets/bobo.jpg").expect("Could not read image");
    let mut img = img.decode().expect("Could not decode image");
    if img.width() > MAX_WIDTH || img.height() > MAX_HEIGHT {
        img = img.resize(MAX_WIDTH, MAX_HEIGHT, imageops::FilterType::Nearest);
    }
    let mut art = AsciiArt::with_width(img.width());
    art.draw(&img);
    art.print();
}
