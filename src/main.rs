pub mod decoder;

use decoder::decode;
use image::{ImageBuffer, Rgba};
use std::{env, fs};

fn main() {
    let argv: Vec<String> = env::args().collect();
    if argv.len() != 2 {
        println!("usage: {} qoi_filename png_filename", argv[0]);
    }

    let qoi_file = String::from(argv[1].clone());
    let input = fs::read(qoi_file).expect("error: fs::read_to_string");

    let img: ImageBuffer<Rgba<u8>, Vec<u8>> = decode::decode_image(&input);
    let _ = img.save("/home/ayush/Desktop/kodim23.png");
}
