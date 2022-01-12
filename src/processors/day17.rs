use regex::Regex;

fn parse_input() -> Target {
    let re = Regex::new(r"-*\d+").unwrap();
    let mut input = re
        .find_iter(include_str!("../../data/day17.txt"))
        .map(|x| x.as_str().parse::<isize>().unwrap());

    Target {
        x_min: input.next().unwrap(),
        x_max: input.next().unwrap(),
        y_min: input.next().unwrap(),
        y_max: input.next().unwrap(),
    }
}

pub fn processor() -> (isize, isize) {
    let target = parse_input();
    let mut output = (0, 0);
    for x in 1..target.x_max + 1 {
        for y in target.y_min..128 {
            let (result, max_y) = evaluate(target, (x, y));
            if result {
                output.1 += 1;
                if max_y > output.0 {
                    output.0 = max_y;
                }
            }
        }
    }

    output
}

fn evaluate(target: Target, base_vel: (isize, isize)) -> (bool, isize) {
    let mut pos = (0, 0);
    let mut vel = base_vel.clone();
    let mut max_y = 0;
    let mut found = false;
    while pos.0 <= target.x_max && pos.1 >= target.y_min {
        pos.0 += vel.0;
        pos.1 += vel.1;

        if vel.0 > 0 {
            vel.0 -= 1
        } else if vel.0 < 0 {
            vel.0 += 1
        }

        if pos.1 > max_y {
            max_y = pos.1
        }

        vel.1 -= 1;

        if target.contains(pos) {
            found = true;
            break;
        }
    }

    (found, max_y)
}

#[derive(Clone, Copy)]
struct Target {
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
}

impl Target {
    pub fn contains(&self, pos: (isize, isize)) -> bool {
        pos.0 >= self.x_min && pos.0 <= self.x_max && pos.1 >= self.y_min && pos.1 <= self.y_max
    }
}
