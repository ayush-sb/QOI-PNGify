use image::{ImageBuffer, Rgba};
use super::header::parse_header;

const RUN_ARRAY_SIZE: u8 = 64;

pub fn decode_image(input: &[u8]) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    /* parse header */
    let (input, header) =
        parse_header(input).expect("Failed to parse header: not a valid qoi image");

    /* initialize data structures */
    let mut image_buffer: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::new(header.width, header.height);
    let mut run_array: Vec<Rgba<u8>> = vec![Rgba([0, 0, 0, 0]); RUN_ARRAY_SIZE.into()];
    let mut prev_pixel = Rgba([0, 0, 0, 255]);

    /* parse input and get chunks */
    let mut chunks_vec: Vec<Rgba<u8>>;
    let mut curr_chunk = 
    while !

    image_buffer
}

fn hash(pixel: &Rgba<u8>) -> u8 {
    let hash_value: u8 =
        (pixel.0[0] * 3 + pixel.0[1] * 5 + pixel.0[2] * 7 + pixel.0[3] * 11) % RUN_ARRAY_SIZE;
    hash_value
}
