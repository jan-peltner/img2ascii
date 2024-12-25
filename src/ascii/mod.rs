use colored::{Color, ColoredString, Colorize};
use image::{DynamicImage, GenericImageView, Pixel, Rgb};

pub fn brighten(img: DynamicImage) -> DynamicImage {
    let mut alpha_matrix: Vec<f32> = Vec::new();
    for pixel in img.pixels() {
        let rgb = pixel.2.to_rgb();
        let alpha = get_perceived_brightness(&rgb);
        alpha_matrix.push(alpha);
    }

    alpha_matrix.sort_by(|a, b| a.total_cmp(b));
    let median_alpha = alpha_matrix[alpha_matrix.len() / 2];
    let alpha_offset = (255 / 2) as f32 - median_alpha;
    img.brighten(alpha_offset as i32)
}

fn get_perceived_brightness(rgb: &Rgb<u8>) -> f32 {
    rgb.0[0] as f32 * 0.299 + rgb.0[1] as f32 * 0.587 + rgb.0[2] as f32 * 0.114
}

#[derive(Debug)]
struct AsciiConf {
    width: u32,
    charset: Vec<char>,
}

#[derive(Debug)]
pub struct AsciiPixel {
    alpha: f32,
    color: Color,
}

#[derive(Debug)]
pub struct AsciiArt {
    conf: AsciiConf,
    matrix: Vec<AsciiPixel>,
}

impl AsciiArt {
    pub fn with_width(width: u32) -> Self {
        let conf = AsciiConf {
            width,
            charset: Vec::from(['.', ':', '=', '*', '#', '@']),
        };
        Self {
            conf,
            matrix: Vec::new(),
        }
    }

    pub fn construct_pixels_from_img(&mut self, image: &DynamicImage) {
        for pixel in image.pixels() {
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
            self.matrix.push(ascii_pixel);
        }
    }

    pub fn draw(&self) {
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
