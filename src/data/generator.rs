use std::ops::Deref;

use super::data::{Cell, CellPixel, Pixel};

pub struct CellGen { }

impl CellGen {
    pub fn gen_all_cells(bg: Pixel, pixels: Vec<CellPixel>) -> Vec<Cell> {
        let mut cells = vec![Cell {
            side: [0, 0, 0, 0],
            data: [bg, bg, bg,
                   bg, bg, bg,
                   bg, bg, bg]
        }];

        for pixel in pixels.iter() {
            cells.append(&mut Self::gen_normal_cells(bg, pixel));
        }

        if pixels.len() > 1 {
            for x in 0..(pixels.len() - 1) {
                let px1 = pixels.iter().nth(x).unwrap();

                for y in (x + 1)..(pixels.len() - 1) {
                    let px2 = pixels.iter().nth(y).unwrap();
                    cells.append(&mut Self::gen_connections(bg, px1, px2));
                }
            }
        }

        return cells;
    }
    
    fn gen_normal_cells(bg: Pixel, px: &CellPixel) -> Vec<Cell> {
        let cl = px.color;

        let cells = vec![
            Cell {
                side: [px.id, px.id, px.id, 0],
                data: [bg, cl, bg,
                       bg, cl, cl,
                       bg, cl, bg]
            }, Cell {
                side: [0, px.id, px.id, px.id],
                data: [bg, bg, bg,
                       cl, cl, cl,
                       bg, cl, bg]
            }, Cell {
                side: [px.id, 0, px.id, px.id],
                data: [bg, cl, bg,
                       cl, cl, bg,
                       bg, cl, bg]
            }, Cell {
                side: [px.id, px.id, 0, px.id],
                data: [bg, cl, bg,
                       cl, cl, cl,
                       bg, bg, bg]
            }, Cell {
                side: [0, 0, px.id, px.id],
                data: [bg, bg, bg,
                       cl, cl, bg,
                       bg, cl, bg]
            }, Cell {
                side: [0, px.id, 0, px.id],
                data: [bg, bg, bg,
                       cl, cl, cl,
                       bg, bg, bg]
            }, Cell {
                side: [0, px.id, px.id, 0],
                data: [bg, bg, bg,
                       bg, cl, cl,
                       bg, cl, bg]
            }, Cell {
                side: [px.id, 0, px.id, 0],
                data: [bg, cl, bg,
                       bg, cl, bg,
                       bg, cl, bg]
            }, Cell {
                side: [px.id, 0, 0, px.id],
                data: [bg, cl, bg,
                       cl, cl, bg,
                       bg, bg, bg]
            }, Cell {
                side: [px.id, px.id, 0, 0],
                data: [bg, cl, bg,
                       bg, cl, cl,
                       bg, bg, bg]
            }, Cell {
                side: [px.id, px.id, px.id, px.id],
                data: [bg, cl, bg,
                       cl, cl, cl,
                       bg, cl, bg]
            }
        ];

        return cells;
    }

    fn gen_connections(bg: Pixel, px1: &CellPixel, px2: &CellPixel) -> Vec<Cell> {
        let c1 = px1.color;
        let c2 = px2.color;

        let cells = vec![
            Cell {
                side: [px1.id, px2.id, px1.id, px2.id],
                data: [bg, c1, bg,
                       c2, c2, c2,
                       bg, c1, bg]
            }, Cell {
                side: [px1.id, px2.id, px1.id, px2.id],
                data: [bg, c1, bg,
                       c2, c2, c2,
                       bg, c1, bg]
            }, Cell {
                side: [px2.id, px1.id, px2.id, px1.id],
                data: [bg, c2, bg,
                       c1, c2, c1,
                       bg, c2, bg]
            }, Cell {
                side: [px2.id, px1.id, px2.id, px1.id],
                data: [bg, c2, bg,
                       c1, c1, c1,
                       bg, c2, bg]
        }];

        return cells;
    }
}