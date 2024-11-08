pub type Pixel = [u8; 3];

pub const WHITE: Pixel = [235, 235, 235];
pub const BLUE:  Pixel = [53,  137, 191];
pub const BLACK: Pixel = [37,  38,  40];
pub const RED:   Pixel = [255,  0,  00];

pub const SIDE_UP:    usize = 0;
pub const SIDE_RIGHT: usize = 1;
pub const SIDE_DOWN:  usize = 2;
pub const SIDE_LEFT:  usize = 3;

#[derive(Clone, Copy)]
pub struct Cell {
    pub data: [Pixel; 9],
    pub side: [u8; 4]
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
pub const CELLS: [Cell; 27] = [
    // Black cell
    CELL0000,
    
    // Blue cells
    CELL0022, CELL0202, CELL0220,
    CELL2020, CELL2002, CELL2200,
    CELL2222, CELL2220, CELL2202,
    CELL2022, CELL0222,

    // White cells
    CELL0011, CELL0101, CELL0110,
    CELL1010, CELL1001, CELL1100,
    CELL1111, CELL1110, CELL1101,
    CELL1011, CELL0111,

    // Connection cells
    CELL2121, CELL2121_V2,
    CELL1212, CELL1212_V2,
];

pub const CELL0000: Cell = Cell {
    side: [0, 0, 0, 0],
    data: [BLACK, BLACK, BLACK,
           BLACK, BLACK, BLACK,
           BLACK, BLACK, BLACK]
};

pub const CELL2220: Cell = Cell {
    side: [2, 2, 2, 0],
    data: [BLACK, BLUE, BLACK,
           BLACK, BLUE, BLUE,
           BLACK, BLUE, BLACK]
};

pub const CELL0222: Cell = Cell {
    side: [0, 2, 2, 2],
    data: [BLACK, BLACK, BLACK,
           BLUE,  BLUE,  BLUE,
           BLACK, BLUE,  BLACK]
};

pub const CELL2022: Cell = Cell {
    side: [2, 0, 2, 2],
    data: [BLACK, BLUE, BLACK,
           BLUE,  BLUE, BLACK,
           BLACK, BLUE, BLACK]
};

pub const CELL2202: Cell = Cell {
    side: [2, 2, 0, 2],
    data: [BLACK, BLUE,  BLACK,
           BLUE,  BLUE,  BLUE,
           BLACK, BLACK, BLACK]
};

pub const CELL0022: Cell = Cell {
    side: [0, 0, 2, 2],
    data: [BLACK, BLACK, BLACK,
           BLUE,  BLUE,  BLACK,
           BLACK, BLUE,  BLACK]
};

pub const CELL0202: Cell = Cell {
    side: [0, 2, 0, 2],
    data: [BLACK, BLACK, BLACK,
           BLUE,  BLUE,  BLUE,
           BLACK, BLACK, BLACK]
};

pub const CELL0220: Cell = Cell {
    side: [0, 2, 2, 0],
    data: [BLACK, BLACK, BLACK,
           BLACK, BLUE,  BLUE,
           BLACK, BLUE,  BLACK]
};

pub const CELL2020: Cell = Cell {
    side: [2, 0, 2, 0],
    data: [BLACK, BLUE, BLACK,
           BLACK, BLUE, BLACK,
           BLACK, BLUE, BLACK]
};

pub const CELL2002: Cell = Cell {
    side: [2, 0, 0, 2],
    data: [BLACK, BLUE,  BLACK,
           BLUE,  BLUE,  BLACK,
           BLACK, BLACK, BLACK]
};

pub const CELL2200: Cell = Cell {
    side: [2, 2, 0, 0],
    data: [BLACK, BLUE,  BLACK,
           BLACK, BLUE,  BLUE,
           BLACK, BLACK, BLACK]
};

pub const CELL2222: Cell = Cell {
    side: [2, 2, 2, 2],
    data: [BLACK, BLUE, BLACK,
           BLUE,  BLUE, BLUE,
           BLACK, BLUE, BLACK]
};

pub const CELL0011: Cell = Cell {
    side: [0, 0, 1, 1],
    data: [BLACK, BLACK, BLACK,
           WHITE, WHITE, BLACK,
           BLACK, WHITE, BLACK]
};

pub const CELL0101: Cell = Cell {
    side: [0, 1, 0, 1],
    data: [BLACK, BLACK, BLACK,
           WHITE, WHITE, WHITE,
           BLACK, BLACK, BLACK]
};

pub const CELL0110: Cell = Cell {
    side: [0, 1, 1, 0],
    data: [BLACK, BLACK, BLACK,
           BLACK, WHITE, WHITE,
           BLACK, WHITE, BLACK]
};

pub const CELL1010: Cell = Cell {
    side: [1, 0, 1, 0],
    data: [BLACK, WHITE, BLACK,
           BLACK, WHITE, BLACK,
           BLACK, WHITE, BLACK]
};

pub const CELL1001: Cell = Cell {
    side: [1, 0, 0, 1],
    data: [BLACK, WHITE, BLACK,
           WHITE, WHITE, BLACK,
           BLACK, BLACK, BLACK]
};

pub const CELL1100: Cell = Cell {
    side: [1, 1, 0, 0],
    data: [BLACK, WHITE, BLACK,
           BLACK, WHITE, WHITE,
           BLACK, BLACK, BLACK]
};

pub const CELL1111: Cell = Cell {
    side: [1, 1, 1, 1],
    data: [BLACK, WHITE, BLACK,
           WHITE, WHITE, WHITE,
           BLACK, WHITE, BLACK]
};

pub const CELL1212: Cell = Cell {
    side: [1, 2, 1, 2],
    data: [BLACK, WHITE, BLACK,
           BLUE,  BLUE, BLUE,
           BLACK, WHITE, BLACK]
};

pub const CELL1212_V2: Cell = Cell {
    side: [1, 2, 1, 2],
    data: [BLACK, WHITE, BLACK,
           BLUE,  BLUE, BLUE,
           BLACK, WHITE, BLACK]
};

pub const CELL2121: Cell = Cell {
    side: [2, 1, 2, 1],
    data: [BLACK, BLUE, BLACK,
           WHITE, BLUE, WHITE,
           BLACK, BLUE, BLACK]
};

pub const CELL2121_V2: Cell = Cell {
    side: [2, 1, 2, 1],
    data: [BLACK, BLUE,  BLACK,
           WHITE, WHITE, WHITE,
           BLACK, BLUE,  BLACK]
};

pub const CELL1110: Cell = Cell {
    side: [1, 1, 1, 0],
    data: [BLACK, WHITE, BLACK,
           BLACK, WHITE, WHITE,
           BLACK, WHITE, BLACK]
};

pub const CELL0111: Cell = Cell {
    side: [0, 1, 1, 1],
    data: [BLACK, BLACK, BLACK,
           WHITE, WHITE, WHITE,
           BLACK, WHITE, BLACK]
};

pub const CELL1011: Cell = Cell {
    side: [1, 0, 1, 1],
    data: [BLACK, WHITE, BLACK,
           WHITE, WHITE, BLACK,
           BLACK, WHITE, BLACK]
};

pub const CELL1101: Cell = Cell {
    side: [1, 1, 0, 1],
    data: [BLACK, WHITE, BLACK,
           WHITE, WHITE, WHITE,
           BLACK, BLACK, BLACK]
};