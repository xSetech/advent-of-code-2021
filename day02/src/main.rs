// AoC 2021 - Day 2

use std::env;
use std::fs;
use std::process;


enum Direction {
    Forward,
    Down,
    Up,
}


struct Command {
    direction: Direction,
    amount: i32,
}


struct Position {
    horizontal: i32,
    depth: i32,
    aim: i32,
}


impl Position {

    fn go_part1(&mut self, direction: &Direction, amount: i32) {

        match direction {
            Direction::Forward => self.horizontal += amount,
            Direction::Down => self.depth += amount,
            Direction::Up => self.depth -= amount,
        }

    }

    fn go_part2(&mut self, direction: &Direction, amount: i32) {

        if matches!(direction, Direction::Forward) {
            self.horizontal += amount;
            self.depth += self.aim * amount;
            return;
        }

        match direction {
            Direction::Down => self.aim += amount,
            Direction::Up => self.aim -= amount,
            _ => (),
        }

    }

}


fn str_to_direction(s: &str) -> Option<Direction> {

    match s {
        "forward" => Some(Direction::Forward),
        "down" => Some(Direction::Down),
        "up" => Some(Direction::Up),
        _ => None,
    }

}


fn part1(input: &Vec<Command>) -> i32 {

    let mut position = Position {
        horizontal: 0,
        depth: 0,
        aim: 0,
    };

    for command in input.iter() {
        position.go_part1(&command.direction, command.amount);
    }

    return position.horizontal * position.depth;

}


fn part2(input: &Vec<Command>) -> i32 {

    let mut position = Position {
        horizontal: 0,
        depth: 0,
        aim: 0,
    };

    for command in input.iter() {
        position.go_part2(&command.direction, command.amount);
    }

    return position.horizontal * position.depth;

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
    let part1_input: Vec<Command> = part1_input_lines.iter().map(
        |l| {
            // e.g. "forward 1" -> ["forward", "1"]
            let mut split_line  = l.split_whitespace();
            let direction_str = split_line.next().expect("error fetching str direction");
            Command {
                direction: str_to_direction(&direction_str).expect(&format!("invalid direction: {}", &direction_str)),
                amount: split_line.next().expect("error fetching int amount").parse().expect("error casting str to int"),
            }

        }
    ).collect();
    if part1_input.len() > 0 {  // let empty file mean "skip"
        let part1_answer = part1(&part1_input);
        println!("Part 1 answer: {}", part1_answer);
        let part2_answer = part2(&part1_input);
        println!("Part 2 answer: {}", part2_answer);
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    /// The example inputs and outputs given by AOC
    #[test]
    fn test_part1_example() {
        let example_inputs = vec![
            Command { direction: Direction::Forward, amount: 5 },
            Command { direction: Direction::Down, amount: 5 },
            Command { direction: Direction::Forward, amount: 8 },
            Command { direction: Direction::Up, amount: 3 },
            Command { direction: Direction::Down, amount: 8 },
            Command { direction: Direction::Forward, amount: 2 },
        ];

        let mut position = Position {
            horizontal: 0,
            depth: 0,
            aim: 0,
        };
        for command in example_inputs.iter() {
            position.go_part1(&command.direction, command.amount);
        }
        assert_eq!(15, position.horizontal);
        assert_eq!(10, position.depth);

        let output = part1(&example_inputs);
        assert_eq!(150, output);
    }

    /// The example inputs and outputs given by AOC
    #[test]
    fn test_part2_example() {
        let example_inputs = vec![
            Command { direction: Direction::Forward, amount: 5 },
            Command { direction: Direction::Down, amount: 5 },
            Command { direction: Direction::Forward, amount: 8 },
            Command { direction: Direction::Up, amount: 3 },
            Command { direction: Direction::Down, amount: 8 },
            Command { direction: Direction::Forward, amount: 2 },
        ];

        let mut position = Position {
            horizontal: 0,
            depth: 0,
            aim: 0,
        };
        for command in example_inputs.iter() {
            position.go_part2(&command.direction, command.amount);
        }
        assert_eq!(15, position.horizontal);
        assert_eq!(60, position.depth);

        let output = part2(&example_inputs);
        assert_eq!(900, output);
    }


}