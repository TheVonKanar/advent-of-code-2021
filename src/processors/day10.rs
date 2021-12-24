fn parse_input() -> Vec<&'static str> {
    include_str!("../../data/day10.txt").lines().collect()
}

pub fn process() -> (usize, usize) {
    let input = parse_input();
    let mut output = (0, 0);

    let openings = vec!['(', '[', '{', '<'];
    let closings = vec![')', ']', '}', '>'];
    let corruption_score_table: Vec<usize> = vec![3, 57, 1197, 25137];
    let mut autocomplete_scores = Vec::new();
    for i in 0..input.len() {
        let mut stack = Vec::new();
        let mut is_corrupted = false;
        for char in input[i].chars() {
            if openings.contains(&char) {
                stack.push(openings.iter().position(|&x| x == char).unwrap());
            } else if !stack.is_empty() {
                let wanted = stack.pop().unwrap();
                let current = closings.iter().position(|&x| x == char).unwrap();
                if !is_corrupted && current != wanted {
                    is_corrupted = true;
                    output.0 += corruption_score_table[current];
                }
            }
        }

        if !is_corrupted {
            let mut score = 0;
            while stack.len() > 0 {
                score *= 5;
                score += stack.pop().unwrap() + 1;
            }

            autocomplete_scores.push(score);
        }
    }

    autocomplete_scores.sort_by(|a, b| a.cmp(b));
    output.1 = autocomplete_scores[(autocomplete_scores.len() / 2)];

    output
}
