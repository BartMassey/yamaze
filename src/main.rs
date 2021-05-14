use yamaze::*;

use image;
use imageproc::drawing;

fn draw_cell(render: &mut image::GrayImage, cell: &yamaze::Cell) {
    let (r, c) = cell.coord;
    let (rs, cs) = (r as f32 * 10., c as f32 * 10.);
    let (re, ce) = (rs + 9., cs + 9.);
    for d in 0..4 {
        if !cell.walls[d] {
            continue;
        }
        let (start, end) = match Maze::DIRNS[d] {
            (0, -1) => ((rs, cs), (rs, ce)),
            (1, 0) => ((rs, ce), (re, ce)),
            (0, 1) => ((re, cs), (re, ce)),
            (-1, 0) => ((rs, cs), (re, cs)),
            d => panic!("unknown direction {:?}", d),
        };
        drawing::draw_line_segment_mut(render, start, end, image::Luma([255]));
    }
}

pub fn main() {
    let maze = yamaze::Maze::new(20, 20);
    let mut render = image::GrayImage::new(200, 200);
    for cell in maze.0.values() {
        draw_cell(&mut render, cell);
    }
    render.save("test.png").unwrap();
}
