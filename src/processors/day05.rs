use grid::*;
use regex::Regex;

//use crate::grid::Grid;

fn parse_input() -> Vec<usize> {
    Regex::new(r"[0-9]+")
        .unwrap()
        .find_iter(include_str!("../../data/day05.txt"))
        .map(|x| x.as_str().parse().unwrap())
        .collect()
}

pub fn process() -> (usize, usize) {
    let input = parse_input();
    (process_grid(&input, true), process_grid(&input, false))
}

fn process_grid(input: &Vec<usize>, skip_diagonal: bool) -> usize {
    let mut result = 0;

    let size = input.iter().max().unwrap() + 1;
    let mut grid = Grid::from_vec(vec![0usize; size * size], size);
    for chunk in input.chunks(4) {
        let mut cursor = (chunk[0], chunk[1]);
        let dest = (chunk[2], chunk[3]);

        if skip_diagonal && cursor.0 != dest.0 && cursor.1 != dest.1 {
            continue;
        }

        while cursor != dest {
            grid[cursor.1][cursor.0] += 1;

            if cursor.0 < dest.0 {
                cursor.0 += 1;
            } else if cursor.0 > dest.0 {
                cursor.0 -= 1;
            }

            if cursor.1 < dest.1 {
                cursor.1 += 1;
            } else if cursor.1 > dest.1 {
                cursor.1 -= 1;
            }
        }

        grid[dest.1][dest.0] += 1;
    }

    for y in 0..grid.rows() {
        for x in 0..grid.cols() {
            if grid[y][x] >= 2 {
                result += 1;
            }
        }
    }

    result
}
