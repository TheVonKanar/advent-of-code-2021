use clap::{App, Arg};

mod grid;
mod processors;
mod tests;

fn main() {
    let matches = App::new("Advent of Code 2021")
        .version("0.5.0")
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
        _ => (0, 0),
    };

    println!("Part 1 = {}", result.0);
    println!("Part 2 = {}", result.1);
}
