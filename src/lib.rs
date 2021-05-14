use std::collections::HashMap;

use rand::{self, seq::SliceRandom, Rng};

type Coord = (isize, isize);

#[derive(Debug, Clone)]
pub struct Cell {
    pub coord: Coord,
    pub walls: [bool; 4],
}

impl Cell {
    pub fn new(coord: Coord) -> Self {
        Self { coord, walls: [true; 4] }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Maze(pub HashMap<Coord, Cell>);

impl Maze {
    pub const DIRNS: [(isize, isize); 4] = [
        (-1, 0),   // north
        (0, 1),   // east
        (1, 0),   // south
        (0, -1),   // west
    ];

    pub fn new<F>(start: Coord, mut clipped: F) -> Self
    where
        F: FnMut(Coord) -> bool
    {
        let mut rng = rand::thread_rng();
        let mut cells: HashMap<Coord, Cell> = HashMap::new();
        cells.insert(start, Cell::new(start));
        let mut open = vec![start];
        let mut dirns = [0, 1, 2, 3];
        let mut neighbors = Vec::with_capacity(4);
        while !open.is_empty() {
            let target = rng.gen_range(0..open.len());
            let (r, c) = open[target];
            dirns.shuffle(&mut rng);
            neighbors.clear();
            for &d in &dirns {
                let (dr, dc) = Maze::DIRNS[d];
                let (nr, nc) = (r + dr, c + dc);
                if clipped((nr, nc)) || cells.contains_key(&(nr, nc)) {
                    continue;
                }
                neighbors.push(d);
            }
            if neighbors.is_empty() {
                open.swap_remove(target);
                continue;
            }
            let neighbor = *neighbors.choose(&mut rng).unwrap();
            cells.get_mut(&(r, c)).unwrap().walls[neighbor] = false;
            let (dr, dc) = Maze::DIRNS[neighbor];
            let neighbor_coord = (
                (r + dr),
                (c + dc),
            );
            let mut cell = Cell::new(neighbor_coord);
            cell.walls[(neighbor + 2) % 4] = false;
            assert!(cells.insert(neighbor_coord, cell).is_none());
            open.push(neighbor_coord);
        }
        Maze(cells)
    }

    pub fn new_rect(rows: usize, cols: usize) -> Self {
        Self::new(
            (0, 0),
            |(r, c)| r < 0 || c < 0 || r >= rows as isize || c >= cols as isize,
        )
    }

    pub fn bbox(&self) -> Option<(Coord, Coord)> {
        if self.0.is_empty() {
            return None;
        }
        let r_min = self.0.keys().map(|&(r, _)| r).min().unwrap();
        let c_min = self.0.keys().map(|&(_, c)| c).min().unwrap();
        let r_max = self.0.keys().map(|&(r, _)| r).max().unwrap();
        let c_max = self.0.keys().map(|&(_, c)| c).max().unwrap();
        Some(((r_min, c_min), (r_max - r_min + 1, c_max - c_min + 1)))
    }
}

#[test]
fn test_complete() {
    let maze = Maze::new(20, 20);
    for r in 0..20 {
        for c in 0..20 {
            assert!(maze.0.contains_key(&(r, c)));
        }
    }
}

#[test]
fn test_walls_consistent() {
    let maze = Maze::new(20, 20);
    for r in 0..20 {
        assert!(maze.0[&(r, 0)].walls[3]);
        assert!(maze.0[&(r, 19)].walls[1]);
    }
    for c in 0..20 {
        assert!(maze.0[&(0, c)].walls[0]);
        assert!(maze.0[&(19, c)].walls[2]);
    }
    for r in 0..20 {
        for c in 0..19 {
            let cell_1 = &maze.0[&(r, c)];
            let cell_2 = &maze.0[&(r, c + 1)];
            let wall_1 = cell_1.walls[1];
            let wall_2 = cell_2.walls[3];
            assert_eq!(wall_1, wall_2);
        }
    }
    for c in 0..20 {
        for r in 0..19 {
            let cell_1 = &maze.0[&(r, c)];
            let cell_2 = &maze.0[&(r + 1, c)];
            let wall_1 = cell_1.walls[2];
            let wall_2 = cell_2.walls[0];
            assert_eq!(wall_1, wall_2);
        }
    }
}
