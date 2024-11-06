use image::{ImageBuffer, RgbImage, Rgb};
use rand::{thread_rng, Rng};

const CELL_SIZE: u32=  3;
const SIZE: u32 = 100;

const WIDTH:  i32 = (SIZE * CELL_SIZE) as i32;
const HEIGHT: i32 = (SIZE * CELL_SIZE) as i32;

// Fast access to basic colors
const WHITE: [u8; 3] = [255, 255, 255];
const BLACK: [u8; 3] = [0, 0, 0];

// So-called images, aka array of pixels
type Img = [[u8; 3]; 9];

const IMAGE_BLACK: Img = [
    BLACK, BLACK, BLACK,
    BLACK, WHITE, BLACK,
    BLACK, BLACK, BLACK
];

const IMAGE_WHITE: Img = [
    WHITE, WHITE, WHITE,
    WHITE, BLACK, WHITE,
    WHITE, WHITE, WHITE
];

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
    let bool = rng.gen_bool(0.5);
    let rng_img = if bool { IMAGE_BLACK }
                                   else { IMAGE_WHITE };

    // Iterating cell, filling it with pixels of random image
    for x in 0..CELL_SIZE {
        for y in 0..CELL_SIZE {
            let pixel_index = (y + CELL_SIZE * x) as usize;
            img.put_pixel(loc_x + x, loc_y + y, Rgb(rng_img[pixel_index]));
        }
    }
}