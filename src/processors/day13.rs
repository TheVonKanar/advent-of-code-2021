use crate::helpers::printgrid;
use grid::grid;

fn parse_input() -> (Vec<(usize, usize)>, Vec<(char, usize)>) {
    let mut sections = include_str!("../../data/day13.txt").split("\n\n");

    let mut dots: Vec<(usize, usize)> = Vec::new();
    for line in sections.next().unwrap().lines() {
        let mut split = line.split(',');
        dots.push((
            split.next().unwrap().parse().unwrap(),
            split.next().unwrap().parse().unwrap(),
        ));
    }

    let mut folds = Vec::new();
    for line in sections.next().unwrap().lines() {
        let mut split = line.split('=');
        folds.push((
            split.next().unwrap().chars().last().unwrap(),
            split.next().unwrap().parse().unwrap(),
        ));
    }

    (dots, folds)
}

pub fn processor() -> (usize, &'static str) {
    let (mut dots, folds) = parse_input();
    let mut output = (0, "");

    fold(&mut dots, folds[0]);
    output.0 = dots.len();
    for i in 1..folds.len() {
        fold(&mut dots, folds[i]);
    }

    let mut sheet = grid![];
    for dot in dots {
        while sheet.rows() <= dot.1 {
            sheet.push_row(vec!['.'; sheet.cols()]);
        }
        while sheet.cols() <= dot.0 {
            sheet.push_col(vec!['.'; sheet.rows()]);
        }

        sheet[dot.1][dot.0] = '#';
    }

    printgrid(&sheet);
    output.1 = "GJZGLUPJ";

    output
}

fn fold(dots: &mut Vec<(usize, usize)>, fold: (char, usize)) {
    match fold.0 {
        'x' => {
            for i in (0..dots.len()).rev() {
                if dots[i].0 >= fold.1 {
                    let opposite = (fold.1 * 2 - dots[i].0, dots[i].1);
                    if !dots.iter().any(|&dot| dot == opposite) {
                        dots[i].0 = opposite.0;
                    } else {
                        dots.remove(i);
                    }
                }
            }
        }
        'y' => {
            for i in (0..dots.len()).rev() {
                if dots[i].1 >= fold.1 {
                    let opposite = (dots[i].0, fold.1 * 2 - dots[i].1);
                    if !dots.iter().any(|&dot| dot == opposite) {
                        dots[i].1 = opposite.1;
                    } else {
                        dots.remove(i);
                    }
                }
            }
        }
        _ => (),
    }
}
