fn main() {
    let input = include_str!("../input.txt");
    let mut output = (0, 0);

    let numbers: Vec<u32> = input.lines().map(|i| i.parse().unwrap()).collect();
    let len = numbers.len();
    for i in 1..len {
        if numbers[i] > numbers[i - 1] {
            output.0 += 1;
        }

        if i < len - 2 {
            let current_sum = numbers[i] + numbers[i + 1] + numbers[i + 2];
            let previous_sum = numbers[i - 1] + numbers[i] + numbers[i + 1];
            if current_sum > previous_sum {
                output.1 += 1;
            }
        }
    }

    println!("Part 1 = {}", output.0);
    println!("Part 2 = {}", output.1);
}
