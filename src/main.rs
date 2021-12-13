use core::panic;
/// Advent of Code 2021

use std::env;
use std::path::Path;
use std::process;

mod utils;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;


fn main() {

    // Arguments required:
    //  * day (int), e.g. "1"
    //  * input (path, str), e.g. "./inputs/day01/part1"
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Invalid number of arguments; please provide the day and path to file containing inputs.");
        process::exit(1);
    }
    let args_day = &args[1];
    let args_input_path = &args[2];

    // Parse argument:  day, as str->int
    let day: i32 = args_day.parse::<i32>().expect("'day' argument must be parsable to i32 (e.g. '1')");

    // Parse argument:  path to input, as str->Path
    let input_path: &Path = Path::new(&args_input_path);

    // Validate argument:  path to inputs
    if !input_path.exists() {
        eprintln!("Path to inputs does not exist!");
        process::exit(1);
    }

    // Act on arguments
    match day {
        1 => day01::main(input_path),
        2 => day02::main(input_path),
        3 => day03::main(input_path),
        4 => day04::main(input_path),
        5 => day05::main(input_path),
        _ => panic!("BUG: missing day (TODO: create a table of completed days)"),
    };

}