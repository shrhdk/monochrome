extern crate image;

use image::{GrayImage, Luma};

const FLOYD_STEINBERG_THRESHOLD: u8 = 127;

pub fn dither(gray: &GrayImage) -> GrayImage {
    let mut mono = (*gray).clone();
    let (width, height) = (mono.width(), mono.height());

    for i in 0..height {
        for j in 0..width {
            // +----+----+----+
            // |    |  * | f1 |
            // +----+----+----+
            // | f2 | f3 | f4 |
            // +----+----+----+
            let err = f64::from(update_pixel(&mut mono, j, i));
            let (j, i) = (i64::from(j), i64::from(i));
            update_neighboring_pixel(&mut mono, j + 1, i, 7.0 / 16.0 * err); // f1
            update_neighboring_pixel(&mut mono, j - 1, i + 1, 3.0 / 16.0 * err); // f2
            update_neighboring_pixel(&mut mono, j, i + 1, 5.0 / 15.0 * err); // f3
            update_neighboring_pixel(&mut mono, j + 1, i + 1, 1.0 / 16.0 * err); // f4
        }
    }

    mono
}

pub fn serpentine_scanning_dither(gray: &GrayImage) -> GrayImage {
    let mut mono = (*gray).clone();
    let (width, height) = (mono.width(), mono.height());

    for i in 0..height {
        if i % 2 == 0 {
            for j in 0..width {
                // +----+----+----+
                // |    |  * | f1 |
                // +----+----+----+
                // | f2 | f3 | f4 |
                // +----+----+----+
                let err = f64::from(update_pixel(&mut mono, j, i));
                let (j, i) = (i64::from(j), i64::from(i));
                update_neighboring_pixel(&mut mono, j + 1, i, 7.0 / 16.0 * err); // f1
                update_neighboring_pixel(&mut mono, j - 1, i + 1, 3.0 / 16.0 * err); // f2
                update_neighboring_pixel(&mut mono, j, i + 1, 5.0 / 15.0 * err); // f3
                update_neighboring_pixel(&mut mono, j + 1, i + 1, 1.0 / 16.0 * err); // f4
            }
        } else {
            for j in (width - 1)..=0 {
                // +----+----+----+
                // | f1 |  * |    |
                // +----+----+----+
                // | f4 | f3 | f2 |
                // +----+----+----+
                let err = f64::from(update_pixel(&mut mono, j, i));
                let (j, i) = (i64::from(j), i64::from(i));
                update_neighboring_pixel(&mut mono, j - 1, i, 7.0 / 16.0 * err); // f1
                update_neighboring_pixel(&mut mono, j + 1, i + 1, 3.0 / 16.0 * err); // f2
                update_neighboring_pixel(&mut mono, j, i + 1, 5.0 / 16.0 * err); // f3
                update_neighboring_pixel(&mut mono, j - 1, i + 1, 1.0 / 16.0 * err); // f4
            }
        }
    }

    mono
}

fn update_pixel(img: &mut GrayImage, j: u32, i: u32) -> i16 {
    let old_value = img.get_pixel(j, i)[0];
    let new_value = if old_value > FLOYD_STEINBERG_THRESHOLD {
        255
    } else {
        0
    };
    img.put_pixel(j, i, Luma([new_value]));
    i16::from(old_value) - i16::from(new_value) // error
}

fn update_neighboring_pixel(img: &mut GrayImage, j: i64, i: i64, a: f64) {
    let width = i64::from(img.width());
    let height = i64::from(img.height());
    if 0 <= j && j < width && 0 <= i && i < height {
        let (j, i) = (j as u32, i as u32);
        let lum = img.get_pixel(j, i)[0] + a as u8;
        img.put_pixel(j, i, Luma([lum]));
    }
}
