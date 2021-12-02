fn main() {
    let input = include_str!("../input.txt");
    let mut output = (0, 0);

    let mut depth = (0, 0);
    let mut aim = 0;
    let mut pos = 0;
    for line in input.lines() {
        let mut split = line.split(" ");
        let cmd = split.next().unwrap();
        let value: i32 = split.next().unwrap().parse().unwrap();
        match cmd {
            "forward" => {
                pos += value;
                depth.1 += aim * value;
            }
            "up" => {
                depth.0 -= value;
                aim -= value;
            }
            "down" => {
                depth.0 += value;
                aim += value
            }
            _ => (),
        }
    }

    output.0 = pos * depth.0;
    output.1 = pos * depth.1;

    println!("Part 1 = {}", output.0);
    println!("Part 2 = {}", output.1);
}
