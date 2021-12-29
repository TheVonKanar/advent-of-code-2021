use std::collections::{HashMap, HashSet};

fn parse_input() -> (Vec<char>, HashMap<(char, char), char>) {
    let mut lines = include_str!("../../data/day14.txt").lines();
    let template = lines.next().unwrap().chars().collect();

    let mut rules = HashMap::new();
    for line in lines.skip(1) {
        let mut split = line.split(" -> ");
        let mut from = split.next().unwrap().chars();
        let mut to = split.next().unwrap().chars();
        rules.insert(
            (from.next().unwrap(), from.next().unwrap()),
            to.next().unwrap(),
        );
    }

    (template, rules)
}

pub fn processor() -> (usize, usize) {
    let (template, rules) = parse_input();
    let mut output = (0, 0);

    let unique_chars = HashSet::<&char>::from_iter(rules.values());

    let mut pairs: HashMap<(char, char), usize> = HashMap::new();
    for window in template.windows(2) {
        let pair = pairs.entry((window[0], window[1])).or_insert(0);
        *pair += 1;
    }

    for step in 0..40 {
        let mut new_pairs: HashMap<(char, char), usize> = HashMap::new();
        for ((start, end), count) in pairs {
            let insert = *rules.get(&(start, end)).unwrap();
            *new_pairs.entry((start, insert)).or_insert(0) += count;
            *new_pairs.entry((insert, end)).or_insert(0) += count;
        }

        pairs = new_pairs;

        if step == 9 {
            output.0 = count_polymer(&pairs, &template, &unique_chars);
        }
    }

    output.1 = count_polymer(&pairs, &template, &unique_chars);

    output
}

fn count_polymer(
    pairs: &HashMap<(char, char), usize>,
    template: &Vec<char>,
    unique_chars: &HashSet<&char>,
) -> usize {
    let mut counts = Vec::new();
    for char in unique_chars {
        let mut char_count = 0;
        for ((start, _), pair_count) in pairs {
            if start == *char {
                char_count += pair_count;
            }
        }

        if template.last() == Some(char) {
            char_count += 1;
        }

        counts.push(char_count);
    }

    counts.sort();
    counts.last().unwrap() - counts.first().unwrap()
}
