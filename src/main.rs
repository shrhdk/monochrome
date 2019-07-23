extern crate clap;
extern crate image;

use clap::{App, Arg};
use image::{GrayImage, ImageBuffer, Luma};

fn main() {
    let app = App::new("monochrome")
        .version("0.1.0")
        .author("Hideki Shiro <hideki@shiro.be>")
        .about("Convert color images to monochrome images with several algorithms.")
        .arg(
            Arg::with_name("algorithm")
                .help("Algorithm 'floyd' (floyd-steinberg) or bayer'")
                .required(true),
        )
        .arg(Arg::with_name("in").help("Input file path").required(true))
        .arg(
            Arg::with_name("out")
                .help("Output file path")
                .required(true),
        )
        .arg(
            Arg::with_name("gamma")
                .help("Gamma correction value")
                .short("g")
                .long("gamma")
                .takes_value(true),
        );

    let matches = app.get_matches();

    let algorithm = matches.value_of("algorithm").unwrap();
    let in_path = matches.value_of("in").unwrap();
    let out_path = matches.value_of("out").unwrap();
    let gamma = clap::value_t!(matches.value_of("gamma"), f64).unwrap_or(1.0);

    let gray = match image::open(in_path) {
        Ok(img) => img.to_luma(),
        Err(e) => {
            eprintln!("Failed to read input image : {}", e);
            return;
        }
    };

    let gray = if (gamma - 1.0).abs() < 0.001 {
        gray
    } else {
        let gamma_lut = create_gamma_lut(gamma);
        apply_lut(gray, &gamma_lut)
    };

    let mono = match algorithm {
        "floyd-steinberg" | "floyd" => floyd_steinberg_dithering(gray),
        "bayer" => bayer_dithering(gray),
        unknown => {
            eprintln!("Unknown algorithm : {}", unknown);
            return;
        }
    };

    if let Err(e) = mono.save(out_path) {
        eprintln!("Failed to save output image : {}", e);
    }
}

fn create_gamma_lut(gamma: f64) -> [u8; 256] {
    let mut lut = [0u8; 256];
    for x in 0..=255u8 {
        lut[x as usize] = (255.0 * (f64::from(x) / 255.0).powf(1.0 / gamma)) as u8;
    }
    lut
}

fn apply_lut(
    src: ImageBuffer<Luma<u8>, Vec<u8>>,
    lut: &[u8; 256],
) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let (width, height) = (src.width(), src.height());
    let mut dst = GrayImage::new(width, height);

    for i in 0..height {
        for j in 0..width {
            let x = src.get_pixel(j, i)[0];
            let y = lut[x as usize];
            dst.put_pixel(j, i, Luma([y]));
        }
    }

    dst
}

const FLOYD_STEINBERG_THRESHOLD: u8 = 127;

fn floyd_steinberg_dithering(
    gray: ImageBuffer<Luma<u8>, Vec<u8>>,
) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let mut mono = gray.clone();
    let (width, height) = (mono.width(), mono.height());

    for i in 0..height {
        for j in 0..width {
            // +----+----+----+
            // |    |  f | f1 |
            // +----+----+----+
            // | f2 | f3 | f4 |
            // +----+----+----+

            let f = mono.get_pixel(j, i)[0];

            let err: f64 = if f > FLOYD_STEINBERG_THRESHOLD {
                mono.put_pixel(j, i, Luma([255]));
                f64::from(f) - 255.0
            } else {
                mono.put_pixel(j, i, Luma([0]));
                f64::from(f)
            };

            // f1
            if j != width - 1 {
                let f1 = mono.get_pixel(j + 1, i)[0] + (7.0 / 16.0 * err) as u8;
                mono.put_pixel(j + 1, i, Luma([f1]));
            }

            // f2
            if j != 0 && i != height - 1 {
                let f2 = mono.get_pixel(j - 1, i + 1)[0] + (3.0 / 16.0 * err) as u8;
                mono.put_pixel(j - 1, i + 1, Luma([f2]));
            }

            // f3
            if i != height - 1 {
                let f3 = mono.get_pixel(j, i + 1)[0] + (5.0 / 15.0 * err) as u8;
                mono.put_pixel(j, i + 1, Luma([f3]));
            }

            // f4
            if j != width - 1 && i != height - 1 {
                let f4 = mono.get_pixel(j + 1, i + 1)[0] + (1.0 / 16.0 * err) as u8;
                mono.put_pixel(j + 1, i + 1, Luma([f4]));
            }
        }
    }

    mono
}

const BAYER_PATTERN: [[u8; 4]; 4] = [[0, 8, 2, 10], [12, 4, 14, 6], [3, 11, 1, 9], [15, 7, 13, 5]];

fn bayer_dithering(gray: ImageBuffer<Luma<u8>, Vec<u8>>) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let mut mono = gray.clone();
    let (width, height) = (mono.width(), mono.height());

    for i in 0..height {
        for j in 0..width {
            let b = BAYER_PATTERN[(i % 4) as usize][(j % 4) as usize] * 16 + 8;
            let f = mono.get_pixel(j, i)[0];
            let f = if b <= f { 255 } else { 0 };
            mono.put_pixel(j, i, Luma([f]));
        }
    }

    mono
}
