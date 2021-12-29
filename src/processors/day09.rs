use grid::*;

fn parse_input() -> Grid<Cell> {
    let mut grid: Grid<Cell> = grid![[]];
    for line in include_str!("../../data/day09.txt").lines() {
        grid.push_row(
            line.chars()
                .map(|x| Cell {
                    height: x.to_string().parse().unwrap(),
                    is_low_point: false,
                    basin: 0,
                })
                .collect(),
        );
    }

    grid
}

pub fn processor() -> (usize, usize) {
    let mut grid = parse_input();
    let mut output = (0, 0);

    for y in 0..grid.rows() {
        for x in 0..grid.cols() {
            let cell = *grid.get(y, x).unwrap();
            if (x == 0 || cell.height < grid[y][x - 1].height)
                && (x >= grid.cols() - 1 || cell.height < grid[y][x + 1].height)
                && (y == 0 || cell.height < grid[y - 1][x].height)
                && (y >= grid.rows() - 1 || cell.height < grid[y + 1][x].height)
            {
                grid[y][x].is_low_point = true;
                output.0 += 1 + cell.height;
            }
        }
    }

    let mut basin_index = 0usize;
    let mut basin_sizes: Vec<usize> = Vec::new();
    for y in 0..grid.rows() {
        for x in 0..grid.cols() {
            basin_index += 1;
            if grid[y][x].is_low_point {
                let mut size = 0;
                process_basin(&mut grid, x, y, basin_index, &mut size);
                basin_sizes.push(size);
            }
        }
    }

    basin_sizes.sort_by(|a, b| b.cmp(a));
    output.1 = basin_sizes
        .chunks(3)
        .next()
        .unwrap()
        .iter()
        .product::<usize>();

    output
}

fn process_basin(grid: &mut Grid<Cell>, x: usize, y: usize, basin_index: usize, size: &mut usize) {
    if grid[y][x].height >= 9 || grid[y][x].basin > 0 {
        return;
    }

    grid[y][x].basin = basin_index;
    *size += 1;

    if x > 0 {
        process_basin(grid, x - 1, y, basin_index, size);
    }

    if x < grid.cols() - 1 {
        process_basin(grid, x + 1, y, basin_index, size);
    }

    if y > 0 {
        process_basin(grid, x, y - 1, basin_index, size);
    }

    if y < grid.rows() - 1 {
        process_basin(grid, x, y + 1, basin_index, size);
    }
}

#[derive(Clone, Copy, Debug)]
struct Cell {
    height: usize,
    is_low_point: bool,
    basin: usize,
}
