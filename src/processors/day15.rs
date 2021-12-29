use grid::{grid, Grid};
use pathfinding::prelude::{absdiff, dijkstra};

fn parse_input() -> Grid<usize> {
    let mut grid = grid![];
    let lines = include_str!("../../data/day15.txt").trim();

    for y in 0..5 {
        for line in lines.lines() {
            let mut row: Vec<usize> = Vec::new();
            for x in 0..5 {
                for char in line.chars() {
                    let risk = char.to_string().parse::<usize>().unwrap() + x + y;
                    row.push(if risk > 9 { risk % 9 } else { risk });
                }
            }

            grid.push_row(row);
        }
    }

    grid
}

pub fn processor() -> (usize, usize) {
    let grid = parse_input();
    let start = Pos(0, 0);
    let end_a = Pos(grid.cols() / 5 - 1, grid.rows() / 5 - 1);
    let end_b = Pos(grid.cols() - 1, grid.rows() - 1);

    (
        dijkstra(&start, |p| p.successors(&grid, &end_a), |p| *p == end_a)
            .unwrap()
            .1
            + absdiff(grid[start.1][start.0], grid[end_a.1][end_a.0]),
        dijkstra(&start, |p| p.successors(&grid, &end_b), |p| *p == end_b)
            .unwrap()
            .1
            + absdiff(grid[start.1][start.0], grid[end_b.1][end_b.0]),
    )
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

impl Pos {
    fn successors(&self, grid: &Grid<usize>, end: &Pos) -> Vec<(Pos, usize)> {
        let &Pos(x, y) = self;
        let mut neighbours = Vec::new();
        if x > 0 {
            neighbours.push(Pos(x - 1, y));
        }
        if x < end.0 {
            neighbours.push(Pos(x + 1, y));
        }
        if y > 0 {
            neighbours.push(Pos(x, y - 1));
        }
        if y < end.1 {
            neighbours.push(Pos(x, y + 1));
        }

        neighbours
            .into_iter()
            .map(|p| {
                let &Pos(x, y) = self;
                (p, grid[y][x])
            })
            .collect()
    }
}
