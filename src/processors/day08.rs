fn parse_input() -> Vec<&'static str> {
    include_str!("../../data/day08.txt").lines().collect()
}

pub fn process() -> (usize, usize) {
    let input = parse_input();
    let mut output = (0, 0);
    for line in input {
        // Prepare input line
        let mut split = line.split(" | ");
        let mut patterns: Vec<&str> = split.next().unwrap().split(' ').collect();
        let digits: Vec<&str> = split.next().unwrap().split(' ').collect();

        // Decode partial configuration, we only need wires A, B and D
        patterns.sort_by(|a, b| a.len().cmp(&b.len()));
        let mut config = vec![' '; 7];

        // Wire A
        for char in patterns[1].chars() {
            if !patterns[0].contains(char) {
                config[0] = char;
                break;
            }
        }

        // Wire B & D
        let mut chars: Vec<char> = patterns[2].chars().collect();
        for i in (0..chars.len()).rev() {
            if patterns[0].contains(chars[i]) {
                chars.remove(i);
            } else if patterns[3].contains(chars[i])
                && patterns[4].contains(chars[i])
                && patterns[5].contains(chars[i])
            {
                config[3] = chars[i];
                chars.remove(i);
            }
        }

        assert_eq!(chars.len(), 1);
        config[1] = chars[0];

        // Process pattern values
        let mut values: Vec<char> = Vec::new();
        for i in 0..patterns.len() {
            match patterns[i].len() {
                2 => values.push('1'),
                3 => values.push('7'),
                4 => values.push('4'),
                5 => {
                    if patterns[i].contains(config[1]) {
                        values.push('5');
                    } else if patterns[i].contains(patterns[0].chars().nth(0).unwrap())
                        && patterns[i].contains(patterns[0].chars().nth(1).unwrap())
                    {
                        values.push('3');
                    } else {
                        values.push('2');
                    }
                }
                6 => {
                    if !patterns[i].contains(config[3]) {
                        values.push('0');
                    } else if patterns[i].contains(patterns[0].chars().nth(0).unwrap())
                        && patterns[i].contains(patterns[0].chars().nth(1).unwrap())
                    {
                        values.push('9');
                    } else {
                        values.push('6');
                    }
                }
                7 => values.push('8'),
                _ => (),
            }
        }

        // Process digits
        let mut code: String = String::new();
        for digit in digits {
            // Part 1
            match digit.len() {
                2 | 3 | 4 | 7 => output.0 += 1,
                _ => (),
            }

            // Part 2
            let mut index = 0;
            for i in 0..patterns.len() {
                if patterns[i].len() == digit.len()
                    && patterns[i].chars().all(|c| digit.contains(c))
                {
                    index = i;
                    break;
                }
            }

            code.push(values[index]);
        }

        output.1 += code.parse::<usize>().unwrap();
    }

    output
}
