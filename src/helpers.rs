#![allow(dead_code)]

use std::fmt::Display;

use grid::Grid;

pub fn abs_diff(lhs: usize, rhs: usize) -> usize {
    if lhs < rhs {
        rhs - lhs
    } else {
        lhs - rhs
    }
}

pub fn printgrid<T>(grid: &Grid<T>)
where
    T: Display,
{
    for y in 0..grid.rows() {
        for x in 0..grid.cols() {
            print!("{}", grid[y][x].to_string());
        }

        print!("\n");
    }
}
