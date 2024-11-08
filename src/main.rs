use wfc::wfc::WFC;

pub mod data;
pub mod wfc;

const CELL_SIZE: usize =  3;
const SIZE:      usize = 20;

fn main() {
    let mut wfc = WFC::new(SIZE, CELL_SIZE);
    wfc.wfc();
}