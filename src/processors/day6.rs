fn parse_input() -> Vec<usize> {
    include_str!("../../data/day6.txt")
        .split(',')
        .map(|x| x.trim().parse().unwrap())
        .collect()
}

pub fn process() -> (usize, usize) {
    let input = parse_input();
    (count_fish(&input, 80), count_fish(&input, 256))
}

fn count_fish(input: &Vec<usize>, duration: usize) -> usize {
    let mut fishes = vec![0; 9];
    for i in 0..input.len() {
        fishes[input[i]] += 1;
    }

    for _ in 0..duration {
        let new = fishes[0];
        for i in 1..fishes.len() {
            fishes[i - 1] = fishes[i];
        }

        fishes[6] += new;
        fishes[8] = new;
    }

    fishes.iter().sum()
}
