use clap::{App, Arg};

mod helpers;
mod processors;
mod tests;

fn main() {
    let matches = App::new("Advent of Code 2021")
        .version("0.12.0")
        .author("TheVonKanar")
        .arg(
            Arg::with_name("PROCESSOR")
                .help("Sets the processor to use")
                .required(true),
        )
        .get_matches();

    let processor = matches.value_of("PROCESSOR").unwrap();
    let result = match processor {
        "1" => processors::day01::process(),
        "2" => processors::day02::process(),
        "3" => processors::day03::process(),
        "4" => processors::day04::process(),
        "5" => processors::day05::process(),
        "6" => processors::day06::process(),
        "7" => processors::day07::process(),
        "8" => processors::day08::process(),
        "9" => processors::day09::process(),
        "10" => processors::day10::process(),
        "11" => processors::day11::process(),
        "12" => processors::day12::process(),
        _ => (0, 0),
    };

    println!("Part 1 = {}", result.0);
    println!("Part 2 = {}", result.1);
}
