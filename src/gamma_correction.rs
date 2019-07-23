extern crate image;

use image::{GrayImage, Luma};

pub fn apply(img: &GrayImage, gamma: f64) -> GrayImage {
    let gamma_lut = create_gamma_lut(gamma);
    apply_lut(&img, &gamma_lut)
}

fn create_gamma_lut(gamma: f64) -> [u8; 256] {
    let mut lut = [0u8; 256];
    for x in 0..=255u8 {
        lut[x as usize] = (255.0 * (f64::from(x) / 255.0).powf(1.0 / gamma)) as u8;
    }
    lut
}

fn apply_lut(img: &GrayImage, lut: &[u8; 256]) -> GrayImage {
    let mut dst = GrayImage::new(img.width(), img.height());
    for i in 0..dst.height() {
        for j in 0..dst.width() {
            let x = img.get_pixel(j, i)[0];
            let y = lut[x as usize];
            dst.put_pixel(j, i, Luma([y]));
        }
    }
    dst
}
