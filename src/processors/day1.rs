fn parse_input() -> Vec<u32> {
    include_str!("../../data/day1.txt")
        .lines()
        .map(|i| i.parse().unwrap())
        .collect()
}

pub fn process() -> (isize, isize) {
    let input = parse_input();
    let mut output = (0, 0);

    let len = input.len();
    for i in 1..len {
        if input[i] > input[i - 1] {
            output.0 += 1;
        }

        if i < len - 2 {
            let current_sum = input[i] + input[i + 1] + input[i + 2];
            let previous_sum = input[i - 1] + input[i] + input[i + 1];
            if current_sum > previous_sum {
                output.1 += 1;
            }
        }
    }

    output
}
