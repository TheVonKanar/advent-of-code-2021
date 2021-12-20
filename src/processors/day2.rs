fn parse_input() -> Vec<&'static str> {
    include_str!("../../data/day2.txt").lines().collect()
}

pub fn process() -> (isize, isize) {
    let input = parse_input();
    let mut output = (0, 0);

    let mut depth = (0, 0);
    let mut aim = 0;
    let mut pos = 0;
    for line in &input {
        let mut split = line.split(" ");
        let cmd = split.next().unwrap();
        let value: isize = split.next().unwrap().parse().unwrap();
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

    output
}
