use clap::{App, Arg};

mod helpers;
mod processors;
mod tests;

fn main() {
    let matches = App::new("Advent of Code 2021")
        .version("0.8.0")
        .author("TheVonKanar")
        .arg(
            Arg::with_name("PROCESSOR")
                .help("Sets the processor to use")
                .required(true),
        )
        .get_matches();

    let processor = matches.value_of("PROCESSOR").unwrap();
    let result = match processor {
        "day1" | "1" => processors::day1::process(),
        "day2" | "2" => processors::day2::process(),
        "day3" | "3" => processors::day3::process(),
        "day4" | "4" => processors::day4::process(),
        "day5" | "5" => processors::day5::process(),
        "day6" | "6" => processors::day6::process(),
        "day7" | "7" => processors::day7::process(),
        "day8" | "8" => processors::day8::process(),
        "day9" | "9" => processors::day9::process(),
        _ => (0, 0),
    };

    println!("Part 1 = {}", result.0);
    println!("Part 2 = {}", result.1);
}
