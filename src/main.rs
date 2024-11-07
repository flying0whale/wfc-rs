use data::data::CELLS;
use image::{ImageBuffer, RgbImage, Rgb};
use rand::{thread_rng, Rng};

pub mod data;

const CELL_SIZE: u32=  3;
const SIZE: u32 = 100;

const WIDTH:  i32 = (SIZE * CELL_SIZE) as i32;
const HEIGHT: i32 = (SIZE * CELL_SIZE) as i32;

fn main() {
    // Creating empty image [WIDTHxHEIGHT]
    let mut img = RgbImage::new(WIDTH as u32, HEIGHT as u32);
    
    // Iterating image with custom step of CELL_SIZE
    for x in (0..WIDTH).step_by(CELL_SIZE as usize) {
        for y in (0..HEIGHT).step_by(CELL_SIZE as usize) {
            put_rng_img(x as u32, y as u32, &mut img);
        }
    }
    
    let _ = img.save("image.png");
}

fn put_rng_img(loc_x: u32, loc_y: u32, img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    let mut rng = thread_rng();
    let rng_cell_index = rng.gen_range(0..7);
    let rng_cell = &CELLS[rng_cell_index];

    // Iterating cell, filling it with pixels of random image
    for x in 0..CELL_SIZE {
        for y in 0..CELL_SIZE {
            let pixel_index = (y + CELL_SIZE * x) as usize;
            img.put_pixel(loc_x + x, loc_y + y, Rgb(rng_cell.data[pixel_index]));
        }
    }
}