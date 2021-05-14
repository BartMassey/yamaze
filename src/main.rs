use yamaze;

use image;
use imageproc::drawing;

fn draw_cell(render: &mut image::GrayImage, cell: &yamaze::Cell, (ro, co): (isize, isize)) {
    let (r, c) = cell.coord;
    let (rs, cs) = ((r - ro) as f32 * 10., (c - co) as f32 * 10.);
    let (re, ce) = (rs + 9., cs + 9.);
    for d in 0..4 {
        if !cell.walls[d] {
            continue;
        }
        let (start, end) = match d {
            0 => ((cs, rs), (ce, rs)),
            1 => ((ce, rs), (ce, re)),
            2 => ((cs, re), (ce, re)),
            3 => ((cs, rs), (cs, re)),
            d => panic!("unknown direction {:?}", d),
        };
        drawing::draw_line_segment_mut(render, start, end, image::Luma([255]));
    }
}

pub fn main() {
    let dim = std::env::args().nth(1).unwrap().parse::<usize>().unwrap() as isize;
    let outer = dim * dim;
    let inner = outer / 16;
    let clipped = |(r, c)| {
        let (r_off, c_off) = (r - dim, c - dim);
        let radius = r_off * r_off + c_off * c_off;
        radius < inner || radius > outer
    };
    let maze = yamaze::Maze::new((1, dim), clipped);
    let (ul, (h, w)) = maze.bbox().unwrap();
    let mut render = image::GrayImage::new(10 * w as u32, 10 * h as u32);
    for cell in maze.0.values() {
        draw_cell(&mut render, cell, ul);
    }
    render.save("test.png").unwrap();
}
