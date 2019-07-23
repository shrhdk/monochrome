extern crate clap;
extern crate image;

use clap::{App, Arg};

mod bayer;
mod floyd_steinberg;
mod gamma_correction;

fn main() {
    let app = App::new("monochrome")
        .version("0.1.0")
        .author("Hideki Shiro <hideki@shiro.be>")
        .about("Convert color images to monochrome images with several algorithms.")
        .arg(
            Arg::with_name("algorithm")
                .help("Available algorithm: 'floyd' (floyd-steinberg), 'floyd2' (floyd-steinberg serpentine scan) or 'bayer'")
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
    let opt_gamma: Option<f64> = match matches.value_of("gamma") {
        Some(gamma_str) => match gamma_str.parse() {
            Ok(gamma) => Some(gamma),
            Err(e) => {
                eprintln!("Failed to parse gamma value as a float number : {}", e);
                return;
            }
        },
        None => None,
    };

    let mut gray = match image::open(in_path) {
        Ok(img) => img.to_luma(),
        Err(e) => {
            eprintln!("Failed to read input image : {}", e);
            return;
        }
    };

    if let Some(gamma) = opt_gamma {
        gray = gamma_correction::apply(&gray, gamma)
    }

    let mono = match algorithm {
        "floyd" => floyd_steinberg::dither(&gray),
        "floyd2" => floyd_steinberg::serpentine_scanning_dither(&gray),
        "bayer" => bayer::dither(&gray),
        unknown => {
            eprintln!("Unknown algorithm : {}", unknown);
            return;
        }
    };

    if let Err(e) = mono.save(out_path) {
        eprintln!("Failed to save output image : {}", e);
    }
}
