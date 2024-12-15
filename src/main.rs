use colored::{Color, ColoredString, Colorize};
use image::{imageops, GenericImageView, ImageReader, Pixel, Rgb};

#[derive(Debug)]
struct AsciiConf {
    width: u32,
    charset: Vec<char>,
}

#[derive(Debug)]
struct AsciiArt {
    conf: AsciiConf,
    matrix: Vec<AsciiPixel>,
}

impl AsciiArt {
    fn with_width(width: u32) -> Self {
        let conf = AsciiConf {
            width,
            charset: Vec::from(['.', ':', '=', '*', '#', '@']),
        };
        Self {
            conf,
            matrix: Vec::new(),
        }
    }

    fn print(&self) {
        let mut row_count = 0;
        for pixel in &self.matrix {
            row_count += 1;
            let char_amount = self.conf.charset.len();
            let char = self.conf.charset
                [(char_amount as f32 * pixel.alpha).min(char_amount as f32 - 1.0) as usize]
                .to_string();
            let colored_char = ColoredString::from(char).color(pixel.color);
            print!("{colored_char}");
            if row_count % self.conf.width == 0 {
                print!("\n");
                row_count = 0;
            }
        }
    }
}

#[derive(Debug)]
struct AsciiPixel {
    alpha: f32,
    color: Color,
}

fn get_perceived_brightness(rgb: &Rgb<u8>) -> f32 {
    rgb.0[0] as f32 * 0.299 + rgb.0[1] as f32 * 0.587 + rgb.0[2] as f32 * 0.114
}

fn main() {
    let img = ImageReader::open("assets/red_moon_penger.jpg").expect("Could not read image");
    let img = img.decode().expect("Could not decode image");
    let img = img.resize(200, 70, imageops::FilterType::Nearest);
    let mut art = AsciiArt::with_width(img.width());
    for pixel in img.pixels() {
        let rgb = pixel.2.to_rgb();
        let alpha = get_perceived_brightness(&rgb);
        let ascii_pixel = AsciiPixel {
            alpha: alpha / 255f32,
            color: Color::TrueColor {
                r: rgb.0[0],
                g: rgb.0[1],
                b: rgb.0[2],
            },
        };
        art.matrix.push(ascii_pixel);
    }
    art.print();
}
