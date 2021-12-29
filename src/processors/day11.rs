use grid::*;
use std::fmt::*;

fn parse_input() -> Grid<Octopus> {
    let mut grid: Grid<Octopus> = grid![[]];
    for line in include_str!("../../data/day11.txt").lines() {
        grid.push_row(
            line.trim()
                .chars()
                .map(|x| Octopus {
                    energy: x.to_string().parse().unwrap(),
                    last_flash: 0,
                })
                .collect(),
        );
    }

    grid
}

pub fn processor() -> (usize, usize) {
    let mut grid = parse_input();
    let mut output = (0, 0);

    let mut step = 1usize;
    while output.1 == 0 {
        for y in 0..grid.rows() {
            for x in 0..grid.cols() {
                process_flash(&mut grid, x, y, step, &mut output.0);
            }
        }

        if grid.iter().all(|o| o.energy == 0) {
            output.1 = step;
        }

        step += 1;
    }

    output
}

fn process_flash(
    grid: &mut Grid<Octopus>,
    x: usize,
    y: usize,
    step: usize,
    flash_count: &mut usize,
) {
    if grid.get(y, x).is_none() || grid[y][x].last_flash == step {
        return;
    }

    grid[y][x].energy += 1;
    if grid[y][x].energy > 9 {
        grid[y][x].last_flash = step;
        grid[y][x].energy = 0;

        if step <= 100 {
            *flash_count += 1;
        }

        process_flash(grid, x.wrapping_sub(1), y, step, flash_count);
        process_flash(grid, x + 1, y, step, flash_count);
        process_flash(grid, x, y.wrapping_sub(1), step, flash_count);
        process_flash(grid, x, y + 1, step, flash_count);
        process_flash(
            grid,
            x.wrapping_sub(1),
            y.wrapping_sub(1),
            step,
            flash_count,
        );
        process_flash(grid, x + 1, y.wrapping_sub(1), step, flash_count);
        process_flash(grid, x.wrapping_sub(1), y + 1, step, flash_count);
        process_flash(grid, x + 1, y + 1, step, flash_count);
    }
}

#[derive(Clone, Copy)]
struct Octopus {
    energy: usize,
    last_flash: usize,
}

impl Display for Octopus {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.energy)
    }
}
