// AoC 2021 - Day 1

use std::env;
use std::fs;
use std::process;


fn part1(input: Vec<i32>) -> i32 {

    let mut r: i32 = 0;
    let mut x: i32 = input[0];
    for i in input.iter().skip(1) {
        if i > &x {
            r += 1;
        }
        x = *i;
    }
    return r;

}


fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        eprintln!("Expected filenames to be provided as arguments for each problem part input...");
        process::exit(1);
    }

    let part1_input_filename: &String = &args[1];
    println!("Will read inputs from this file for part 1:  {}", part1_input_filename);
    let part1_input_string: String = fs::read_to_string(part1_input_filename).expect("Failed to read file contents for part 1 as a string");
    let part1_input_lines: Vec<&str> = part1_input_string.lines().collect();
    let part1_input: Vec<i32> = part1_input_lines.iter().map(
        |l| l.parse().expect("error in input file")
    ).collect();
    if part1_input.len() > 0 {  // let empty file mean "skip"
        let part1_answer = part1(part1_input);
        println!("Part 1 answer: {}", part1_answer);
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    /// The example inputs and outputs given by AOC
    #[test]
    fn test_part1_example() {
        let example_inputs = [
            199, // (N/A - no previous measurement)
            200, // (increased)
            208, // (increased)
            210, // (increased)
            200, // (decreased)
            207, // (increased)
            240, // (increased)
            269, // (increased)
            260, // (decreased)
            263, // (increased)
        ];
        let output = part1(example_inputs.to_vec());
        assert_eq!(7, output);
    }

}