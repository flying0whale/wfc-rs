pub type Pixel = [u8; 3];

pub const BLACK:    Pixel = [37,  38,  40];
pub const WHITE:    Pixel = [235, 235, 235];
pub const BLUE:     Pixel = [53,  137, 191];
pub const GREEN:    Pixel = [40,  188, 16];
pub const MAGENTA:  Pixel = [163, 27,  183];

pub const RED:    Pixel = [255,  0,  00];

pub const SIDE_UP:    usize = 0;
pub const SIDE_RIGHT: usize = 1;
pub const SIDE_DOWN:  usize = 2;
pub const SIDE_LEFT:  usize = 3;

#[derive(Clone, Copy)]
pub struct Cell {
    pub data: [Pixel; 9],
    pub side: [u8; 4]
}

pub struct CellPixel {
    pub color: Pixel,
    pub id: u8
}

impl Default for Cell {
    fn default() -> Self {
        return Self::new([
            BLACK, BLACK, BLACK,
            BLACK, BLACK, BLACK,
            BLACK, BLACK, BLACK
        ], [255, 255, 255, 255]);
    }
}

impl Cell {
    pub fn new(data: [Pixel; 9], side: [u8; 4]) -> Self {
        return Cell { data, side };
    }
}