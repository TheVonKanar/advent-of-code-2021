use std::{fmt::Display, time::Instant};

use clap::{App, Arg};

mod helpers;
mod processors;
mod tests;

fn main() {
    let matches = App::new("Advent of Code 2021")
        .version("0.15.0")
        .author("TheVonKanar")
        .arg(
            Arg::with_name("DAY")
                .help("Sets the day to process")
                .required(true),
        )
        .get_matches();

    let day = matches.value_of("DAY").unwrap();
    match day {
        "1" => execute(processors::day01::processor),
        "2" => execute(processors::day02::processor),
        "3" => execute(processors::day03::processor),
        "4" => execute(processors::day04::processor),
        "5" => execute(processors::day05::processor),
        "6" => execute(processors::day06::processor),
        "7" => execute(processors::day07::processor),
        "8" => execute(processors::day08::processor),
        "9" => execute(processors::day09::processor),
        "10" => execute(processors::day10::processor),
        "11" => execute(processors::day11::processor),
        "12" => execute(processors::day12::processor),
        "13" => execute(processors::day13::processor),
        "14" => execute(processors::day14::processor),
        "15" => execute(processors::day15::processor),
        "16" => execute(processors::day16::processor),
        "17" => execute(processors::day17::processor),
        _ => (),
    }
}

fn execute<R1, R2>(processor: fn() -> (R1, R2))
where
    R1: Display,
    R2: Display,
{
    let now = Instant::now();
    let result = processor();
    println!("Part 1 = {}", result.0);
    println!("Part 2 = {}", result.1);
    println!("Duration = {}ms", now.elapsed().as_millis());
}
