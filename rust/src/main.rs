use std::fs;
use std::io::Write;
use std::path::Path;
use std::time::Instant;

extern crate clap;
extern crate image;
extern crate rayon;
extern crate webp;

use clap::{Arg, Command};
use rayon::prelude::*;
use webp::Encoder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();

    let matches = Command::new("Image Compressor")
        .version("0.0.1")
        .author("Rust webp image compression tool")
        .about("Compresses images to WEBP format")
        .arg(
            Arg::new("input-dir")
                .long("input-dir")
                .help("Sets the input directory")
                .default_value("./input"),
        )
        .arg(
            Arg::new("output-dir")
                .long("output-dir")
                .help("Sets the output directory")
                .default_value("./output"),
        )
        .arg(
            Arg::new("quality")
                .long("quality")
                .help("Sets the compression quality")
                .default_value("100"),
        )
        .get_matches();

    let input_dir = matches.get_one::<String>("input-dir").unwrap();
    let output_dir = matches.get_one::<String>("output-dir").unwrap();
    let quality = matches
        .get_one::<String>("quality")
        .unwrap()
        .parse::<f32>()
        .unwrap();

    fs::create_dir_all(&output_dir)?;

    let entries = fs::read_dir(input_dir)?
        .filter_map(Result::ok)
        .filter(|entry| {
            entry
                .path()
                .extension()
                .and_then(std::ffi::OsStr::to_str)
                .map(|ext| image::ImageFormat::from_extension(ext).is_some()) // only process supported image formats
                .unwrap_or(false)
        })
        .collect::<Vec<_>>();

    entries.par_iter().for_each(|entry| {
        if let Ok(img) = image::open(&entry.path()) {
            let output_path = Path::new(&output_dir)
                .join(entry.path().file_stem().unwrap())
                .with_extension("webp");

            let rgba_image = img.to_rgba8();
            let width = img.width();
            let height = img.height();

            let webp_data = Encoder::from_rgba(&rgba_image, width, height).encode(quality);

            let mut output_file = fs::File::create(output_path).unwrap();
            output_file.write_all(&webp_data).unwrap();
        }
    });

    let duration = start.elapsed();
    println!("Image compression completed in {:?}", duration);
    Ok(())
}
