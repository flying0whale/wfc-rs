pub type Pixel = [u8; 3];

pub const WHITE: Pixel = [255, 255, 255];
pub const BLACK: Pixel = [0, 0, 0];

pub const SIDE_UP:    usize = 0;
pub const SIDE_RIGHT: usize = 1;
pub const SIDE_DOWN:  usize = 2;
pub const SIDE_LEFT:  usize = 3;

pub struct Cell {
    pub data: [Pixel; 9],
    pub side: [u8; 4]
}

/*
    TABLE OF SIDES (b = black; w = white)
    www: 0
    wbw: 1
*/

impl Default for Cell {
    fn default() -> Self {
        return Self::new([
            WHITE, WHITE, WHITE,
            WHITE, WHITE, WHITE,
            WHITE, WHITE, WHITE
        ], [0, 0, 0, 0]);
    }
}

impl Cell {
    pub fn new(data: [Pixel; 9], side: [u8; 4]) -> Self {
        return Cell { data, side };
    }
}
pub const CELLS: [Cell; 7] = [
    CELL0000, CELL0022, CELL0202,
    CELL0220, CELL2020, CELL2002,
    CELL2200
];

pub const CELL0000: Cell = Cell {
    side: [0, 0, 0, 0],
    data: [WHITE, WHITE, WHITE,
           WHITE, WHITE, WHITE,
           WHITE, WHITE, WHITE]
};

pub const CELL0022: Cell = Cell {
    side: [0, 0, 2, 2],
    data: [WHITE, WHITE, WHITE,
           BLACK, BLACK, WHITE,
           WHITE, BLACK, WHITE]
};

pub const CELL0202: Cell = Cell {
    side: [0, 2, 0, 2],
    data: [WHITE, WHITE, WHITE,
           BLACK, BLACK, BLACK,
           WHITE, WHITE, WHITE]
};

pub const CELL0220: Cell = Cell {
    side: [0, 2, 2, 0],
    data: [WHITE, WHITE, WHITE,
           WHITE, BLACK, BLACK,
           WHITE, BLACK, WHITE]
};

pub const CELL2020: Cell = Cell {
    side: [2, 0, 2, 0],
    data: [WHITE, BLACK, WHITE,
           WHITE, BLACK, WHITE,
           WHITE, BLACK, WHITE]
};

pub const CELL2002: Cell = Cell {
    side: [2, 0, 0, 2],
    data: [WHITE, BLACK, WHITE,
           BLACK, BLACK, WHITE,
           WHITE, WHITE, WHITE]
};

pub const CELL2200: Cell = Cell {
    side: [2, 2, 0, 0],
    data: [WHITE, BLACK, WHITE,
           WHITE, BLACK, BLACK,
           WHITE, WHITE, WHITE]
};