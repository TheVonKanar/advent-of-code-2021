fn parse_input() -> String {
    let mut input = String::new();
    for char in include_str!("../../data/day16.txt").trim().chars() {
        match char {
            '0' => input.push_str("0000"),
            '1' => input.push_str("0001"),
            '2' => input.push_str("0010"),
            '3' => input.push_str("0011"),
            '4' => input.push_str("0100"),
            '5' => input.push_str("0101"),
            '6' => input.push_str("0110"),
            '7' => input.push_str("0111"),
            '8' => input.push_str("1000"),
            '9' => input.push_str("1001"),
            'A' => input.push_str("1010"),
            'B' => input.push_str("1011"),
            'C' => input.push_str("1100"),
            'D' => input.push_str("1101"),
            'E' => input.push_str("1110"),
            'F' => input.push_str("1111"),
            _ => (),
        };
    }

    input
}

pub fn processor() -> (usize, usize) {
    let input = parse_input();
    let mut index = 0usize;

    extract_packet(&input, &mut index)
}

fn extract(input: &String, index: &mut usize, len: usize) -> String {
    let start = *index;
    let end = start + len;
    let result = input[start..end].to_string();
    *index += len;
    result
}

fn extract_packet(input: &String, index: &mut usize) -> (usize, usize) {
    let version = usize::from_str_radix(&extract(&input, index, 3), 2).unwrap();
    let type_id = usize::from_str_radix(&extract(&input, index, 3), 2).unwrap();

    let (add_version, value) = match type_id {
        0 => extract_operator(input, index, |a, b| a + b),
        1 => extract_operator(input, index, |a, b| a * b),
        2 => extract_operator(input, index, |a, b| a.min(b)),
        3 => extract_operator(input, index, |a, b| a.max(b)),
        4 => {
            let mut number = String::new();
            loop {
                let group = extract(&input, index, 5);
                let (prefix, bits) = group.split_at(1);
                number.push_str(bits);
                if prefix == "0" {
                    break;
                }
            }
            (0, usize::from_str_radix(&number, 2).unwrap())
        }
        5 => extract_operator(input, index, |a, b| if a > b { 1 } else { 0 }),
        6 => extract_operator(input, index, |a, b| if a < b { 1 } else { 0 }),
        7 => extract_operator(input, index, |a, b| if a == b { 1 } else { 0 }),
        _ => (0, 0),
    };

    // println!("type id = {}", type_id);
    // println!("value = {}", value);

    (version + add_version, value)
}

fn extract_operator(
    input: &String,
    index: &mut usize,
    value_processor: fn(usize, usize) -> usize,
) -> (usize, usize) {
    let mut version = 0;
    let length_type_id = extract(&input, index, 1);
    let mut value = 0;
    match length_type_id.as_str() {
        "0" => {
            let length = usize::from_str_radix(&extract(&input, index, 15), 2).unwrap();
            let starting_index = *index;
            let mut i = 0;
            loop {
                let (sub_version, sub_value) = extract_packet(input, index);
                version += sub_version;
                value = if i == 0 {
                    sub_value
                } else {
                    value_processor(value, sub_value)
                };

                if *index - starting_index >= length {
                    break;
                }

                i += 1;
            }
        }
        "1" => {
            let length = usize::from_str_radix(&extract(&input, index, 11), 2).unwrap();
            for i in 0..length {
                let (sub_version, sub_value) = extract_packet(input, index);
                version += sub_version;
                value = if i == 0 {
                    sub_value
                } else {
                    value_processor(value, sub_value)
                };
            }
        }
        _ => (),
    }

    (version, value)
}
