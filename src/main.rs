pub mod decoder;

use decoder::decode;
use image::{ImageBuffer, Rgba};
use std::{env, fs};

fn main() {
    let argv: Vec<String> = env::args().collect();
    if argv.len() != 2 && argv.len() != 3 {
        println!("usage: {} qoi_filename [png_filename]", argv[0]);
        return;
    }

    let input = fs::read(argv[1].clone()).expect("error: fs::read_to_string");
    let img: ImageBuffer<Rgba<u8>, Vec<u8>> = decode::decode_image(&input);
    if argv.len() == 2 {
        let name_length = argv[1].len();
        let mut name = String::from(&argv[1][0..(name_length - 3)]);
        name.push_str("png");
        let _ = img.save(name);
    } else {
        let _ = img.save(argv[2].clone());
    }
}
