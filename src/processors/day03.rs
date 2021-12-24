fn parse_input() -> Vec<&'static str> {
    include_str!("../../data/day03.txt").lines().collect()
}

pub fn process() -> (usize, usize) {
    let input = parse_input();
    let mut output = (0, 0);

    // Gamma & Epsilon rates.
    let mut zero_counts = [0; 12];
    let mut one_counts = [0; 12];
    let mut char_index;
    for line in &input {
        char_index = 0;
        for char in line.chars() {
            if char == '0' {
                zero_counts[char_index] += 1;
            } else {
                one_counts[char_index] += 1;
            }

            char_index += 1;
        }
    }

    let mut gamma_bits = String::new();
    let mut epsilon_bits = String::new();
    for i in 0..zero_counts.len() {
        if one_counts[i] > zero_counts[i] {
            gamma_bits.push('1');
            epsilon_bits.push('0');
        } else {
            epsilon_bits.push('1');
            gamma_bits.push('0');
        }
    }

    let gamma_rate = usize::from_str_radix(gamma_bits.as_str(), 2).unwrap();
    let epsilon_rate = usize::from_str_radix(epsilon_bits.as_str(), 2).unwrap();
    output.0 = gamma_rate * epsilon_rate;

    // Oxygen Generator rating.
    char_index = 0;
    let mut candidates = input.to_vec();
    while candidates.len() > 1 {
        let half_count = candidates.len() / 2;
        let wanted = if candidates
            .iter()
            .filter(|x| x.chars().nth(char_index).unwrap() == '1')
            .count()
            >= half_count
        {
            '1'
        } else {
            '0'
        };

        candidates.retain(|&x| x.chars().nth(char_index).unwrap() == wanted);

        char_index += 1;
    }

    let oxygen_rating = usize::from_str_radix(candidates.last().unwrap(), 2).unwrap();

    // CO2 Scrubber rating
    char_index = 0;
    candidates = input.to_vec();
    while candidates.len() > 1 {
        let half_count = candidates.len() / 2;
        let wanted = if candidates
            .iter()
            .filter(|x| x.chars().nth(char_index).unwrap() == '1')
            .count()
            < half_count
        {
            '1'
        } else {
            '0'
        };

        candidates.retain(|&x| x.chars().nth(char_index).unwrap() == wanted);

        char_index += 1;
    }

    let co2_rating = usize::from_str_radix(candidates.last().unwrap(), 2).unwrap();
    output.1 = oxygen_rating * co2_rating;

    output
}
