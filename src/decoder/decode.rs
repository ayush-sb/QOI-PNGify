use super::chunks::{get_all_chunks, CHUNK};
use image::{ImageBuffer, Rgba};
use wrapping_arithmetic::wrappit;

const RUN_ARRAY_SIZE: u8 = 64;
const DIFF_BIAS: u8 = 2;
const LUMA_DG_BIAS: u8 = 32;
const LUMA_DR_DB_BIAS: u8 = 8;
const RUN_BIAS: u8 = 1;

pub fn decode_image(input: &[u8]) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    /* parse header and chunks */
    let (_, (header, chunk_array)) = get_all_chunks(input).expect("Could not parse file");

    /* initialize data structures */
    let mut image_buffer: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::new(header.height, header.width);
    let mut run_array: Vec<Rgba<u8>> = vec![Rgba([0, 0, 0, 0]); RUN_ARRAY_SIZE.into()];
    let mut prev_pixel = Rgba([0, 0, 0, 255]);
    let mut curr_pixel: Rgba<u8>;
    let mut count = 0;

    for chunk in chunk_array {
        match chunk {
            CHUNK::RGBChunk(rgb_chunk) => {
                let r = rgb_chunk.r;
                let g = rgb_chunk.g;
                let b = rgb_chunk.b;
                let a = prev_pixel.0[3];

                curr_pixel = Rgba([r, g, b, a]);
                image_buffer.put_pixel(count / header.width, count % header.width, curr_pixel);
                run_array[hash(&curr_pixel) as usize] = curr_pixel;
                prev_pixel = curr_pixel;
                count = count + 1;
            }

            CHUNK::RGBAChunk(rgba_chunk) => {
                let r = rgba_chunk.r;
                let g = rgba_chunk.g;
                let b = rgba_chunk.b;
                let a = rgba_chunk.a;

                curr_pixel = Rgba([r, g, b, a]);
                image_buffer.put_pixel(count / header.width, count % header.width, curr_pixel);
                run_array[hash(&curr_pixel) as usize] = curr_pixel;
                prev_pixel = curr_pixel;
                count = count + 1;
            }

            CHUNK::IndexChunk(index_chunk) => {
                curr_pixel = run_array[index_chunk.index as usize].clone();
                image_buffer.put_pixel(count / header.width, count % header.width, curr_pixel);
                prev_pixel = curr_pixel;
                count = count + 1;
            }

            CHUNK::DiffChunk(diff_chunk) => {
                let r = prev_pixel.0[0]
                    .wrapping_add(diff_chunk.dr)
                    .wrapping_sub(DIFF_BIAS);
                let g = prev_pixel.0[1]
                    .wrapping_add(diff_chunk.dg)
                    .wrapping_sub(DIFF_BIAS);
                let b = prev_pixel.0[2]
                    .wrapping_add(diff_chunk.db)
                    .wrapping_sub(DIFF_BIAS);
                let a = prev_pixel.0[3];

                curr_pixel = Rgba([r, g, b, a]);
                image_buffer.put_pixel(count / header.width, count % header.width, curr_pixel);
                run_array[hash(&curr_pixel) as usize] = curr_pixel;
                prev_pixel = curr_pixel;
                count = count + 1;
            }

            CHUNK::LumaChunk(luma_chunk) => {
                let dg = luma_chunk.dg.wrapping_sub(LUMA_DG_BIAS);
                let dr_dg = luma_chunk.dr_dg.wrapping_sub(LUMA_DR_DB_BIAS);
                let db_dg = luma_chunk.db_dg.wrapping_sub(LUMA_DR_DB_BIAS);

                let g = dg.wrapping_add(prev_pixel.0[1]);
                let r = dr_dg.wrapping_add(prev_pixel.0[0]).wrapping_add(dg);
                let b = db_dg.wrapping_add(prev_pixel.0[2]).wrapping_add(dg);
                let a = prev_pixel.0[3];

                curr_pixel = Rgba([r, g, b, a]);
                image_buffer.put_pixel(count / header.width, count % header.width, curr_pixel);
                run_array[hash(&curr_pixel) as usize] = curr_pixel;
                prev_pixel = curr_pixel;
                count = count + 1;
            }

            CHUNK::RunChunk(run_chunk) => {
                let num_iters = run_chunk.run.wrapping_add(RUN_BIAS);
                for _ in 0..num_iters {
                    curr_pixel = prev_pixel.clone();
                    image_buffer.put_pixel(count / header.width, count % header.width, curr_pixel);
                    count = count + 1;
                }
            }
        }
    }

    image_buffer
}

#[wrappit]
fn hash(pixel: &Rgba<u8>) -> u8 {
    let hash_value: u8 =
        (pixel.0[0] * 3 + pixel.0[1] * 5 + pixel.0[2] * 7 + pixel.0[3] * 11) % RUN_ARRAY_SIZE;
    hash_value
}
