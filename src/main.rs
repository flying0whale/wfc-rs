use data::{data::{CellPixel, BLACK, BLUE, GREEN, MAGENTA, WHITE}, generator::CellGen};
use wfc::wfc::WFC;

pub mod data;
pub mod wfc;

const CELL_SIZE: usize =  3;
const SIZE:      usize = 50;

fn main() {
    let pixels = vec![
        CellPixel { color: WHITE,    id: 1 },
        CellPixel { color: BLUE,     id: 2 },
        //CellPixel { color: GREEN,    id: 3 },
        //CellPixel { color: MAGENTA,  id: 4 }
    ];

    let cells = CellGen::gen_all_cells(BLACK, pixels, 5);

    let mut wfc = WFC::new(SIZE, CELL_SIZE, cells);
    wfc.wfc();
}