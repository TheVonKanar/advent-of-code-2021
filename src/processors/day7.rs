use crate::helpers::abs_diff;

fn parse_input() -> Vec<usize> {
    include_str!("../../data/day7.txt")
        .split(',')
        .map(|x| x.trim().parse().unwrap())
        .collect()
}

pub fn process() -> (usize, usize) {
    let input = parse_input();
    let mut output = (usize::MAX, usize::MAX);
    let range = *input.iter().min().unwrap()..*input.iter().max().unwrap();
    for i in range {
        let mut cost: (usize, usize) = (0, 0);
        for crab in &input {
            let dist = abs_diff(i, *crab);
            cost.0 += dist;
            cost.1 += dist * (dist + 1) / 2;
        }

        output.0 = usize::min(cost.0, output.0);
        output.1 = usize::min(cost.1, output.1);
    }

    output
}
