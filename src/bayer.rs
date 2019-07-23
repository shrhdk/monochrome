extern crate image;

use image::{GrayImage, Luma};

const BAYER_PATTERN: [[u8; 4]; 4] = [[0, 8, 2, 10], [12, 4, 14, 6], [3, 11, 1, 9], [15, 7, 13, 5]];

pub fn dither(gray: &GrayImage) -> GrayImage {
    let mut mono = (*gray).clone();
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
