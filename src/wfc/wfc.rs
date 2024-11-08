use image::{ImageBuffer, Rgb, RgbImage};
use rand::{thread_rng, Rng};

use crate::data::data::{Cell, CELLS, SIDE_DOWN, SIDE_LEFT, SIDE_RIGHT, SIDE_UP, CELL0220};

pub struct WFC {
    pub size: usize,
    pub cell_size: usize
}

impl WFC {
    pub fn new(size: usize, cell_size: usize) -> Self {
        return WFC {
            size,
            cell_size
        };
    }

    pub fn wfc(&mut self) {
        let mut field: Vec<Cell> = vec![];
        let rand_cell = CELL0220;

        field.push(rand_cell);

        for x in 0..self.size {
            for y in 0..self.size {
                if x == 0 && y == 0 { continue; }

                let cell = self.decide_cell(x, y, field.clone());
                field.push(cell);
            }
        }

        let img = self.create_img(field);
        let _ = img.save("image.png");
    }

    fn create_img(&self, field: Vec<Cell>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let size = self.size * self.cell_size;
        let mut img = RgbImage::new(size as u32, size as u32);

        for x in 0..self.size {
            for y in 0..self.size {
                //println!("DEBUG: x: {x}; y: {y}");
                let cell = &field[y + self.size * x];

                self.put_cell(&mut img, cell, x * self.cell_size, y * self.cell_size);
            }
        }

        return img;
    }

    fn put_cell(&self, img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, cell: &Cell, loc_x: usize, loc_y: usize) {
        for x in 0..self.cell_size {
            for y in 0..self.cell_size {
                let pixel_index = y + self.cell_size * x;
                img.put_pixel((x + loc_x) as u32, (y + loc_y) as u32, Rgb(cell.data[pixel_index]));
            }
        }
    }

    fn decide_cell(&self, x: usize, y: usize, field: Vec<Cell>) -> Cell {
        let next_cell: Cell;

        let neighbors = self.find_neighbors(x, y, &field);
        let n_up   = neighbors[0];
        let n_left = neighbors[1];

        let mut possible_cells: Vec<Cell> = CELLS.clone().to_vec();

        if n_up.is_some() {
            possible_cells = possible_cells.into_iter().filter(|c| {
                c.side[SIDE_UP] == n_up.unwrap().side[SIDE_DOWN]
            }).collect();
        }

        if n_left.is_some() {
            possible_cells = possible_cells.into_iter().filter(|c| {
                c.side[SIDE_LEFT] == n_left.unwrap().side[SIDE_RIGHT]
            }).collect();
        }

        if possible_cells.len() == 0 {
            panic!("NO CELLS HELP");
        }

        /*println!("\nX:{x}; Y:{y}; Possible cells:");
        for c in &possible_cells {
            print!("{:?}", c.side);
            let _ = stdout().flush();
        }*/

        let mut rng = thread_rng();
        next_cell = possible_cells[rng.gen_range(0..possible_cells.len())];

        return next_cell;
    }

    fn find_neighbors(&self, x: usize, y: usize, field: &Vec<Cell>) -> [Option<Cell>; 2] {
        let current = y + x * self.size;

        let left    = field.iter().nth(current - 1).map(|v| *v);
        let up      = field.iter().nth(((current as i32) - (self.size as i32)) as usize).map(|v| *v);

        return [up, left];
    }
}