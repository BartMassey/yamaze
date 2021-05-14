use yamaze;

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
        let (start, end) = match d {
            0 => ((rs, cs), (rs, ce)),
            1 => ((rs, ce), (re, ce)),
            2 => ((re, cs), (re, ce)),
            3 => ((rs, cs), (re, cs)),
            d => panic!("unknown direction {:?}", d),
        };
        drawing::draw_line_segment_mut(render, start, end, image::Luma([255]));
    }
}

pub fn main() {
    let dim: usize = std::env::args().nth(1).unwrap().parse().unwrap();
    let outer = (dim * dim) as isize;
    let inner = outer / 4;
    let clipped = |(r, c)| {
        let (r_off, c_off) = (r - dim as isize, c - dim as isize);
        let radius = r_off * r_off + c_off * c_off;
        radius < inner || radius > outer
    };
    let maze = yamaze::Maze::new((1, dim), clipped);
    let mut render = image::GrayImage::new(10 * dim as u32, 10 * dim as u32);
    for cell in maze.0.values() {
        draw_cell(&mut render, cell);
    }
    render.save("test.png").unwrap();
}
