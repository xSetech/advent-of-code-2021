// AoC 2021 - Day 4

use std::collections::HashMap;
use std::fs;
use std::path::Path;


mod bingo {

    pub const ROWS: usize = 5;
    pub const COLS: usize = 5;

    use super::*;

    pub struct Input {
        pub numbers: Vec<u32>,
        pub boards: Vec<Board>,
    }

    struct Cell {
        value: u32,
        hit: bool,
    }

    pub struct Board {
        pub grid: [[u32; COLS]; ROWS],
    }

    /// How a board will perform given a sequence of numbers to be played in a
    /// game of Bingo. Predictions for Boards can be compared against one
    /// another to determine which would win a given game, at what time, and
    /// with what score.
    pub struct Prediction {
        pub when: usize,
        pub score: u32,
    }

    impl Board {

        /// Check the given coordinate's row and column for Bingo!
        fn check(&self, cells: &[[Cell; bingo::COLS]; bingo::ROWS], coord: &(usize, usize)) -> bool {
            if cells.iter().filter( |row| { row[coord.1].hit } ).count() == bingo::ROWS {
                return true;
            }
            if cells[coord.0].iter().filter( |col| { col.hit } ).count() == bingo::COLS {
                return true;
            }
            return false;
        }

        /// Compute the board's current score given the winning number
        fn score(&self, cells: &[[Cell; bingo::COLS]; bingo::ROWS], number: u32) -> u32 {
            let sum: u32 = cells
                .iter()
                .flat_map(|row| { row.iter() })
                .filter(|cell| { !cell.hit })
                .map(|cell| { cell.value })
                .sum();
            return sum * number;
        }

        /// Map all values on the board to their coordinate.
        /// This builds an index for looking up coordinates by value.
        fn index(&self, cells: &[[Cell; bingo::COLS]; bingo::ROWS]) -> HashMap<u32, Vec<(usize, usize)>> {

            let mut values_to_coords: HashMap<u32, Vec<(usize, usize)>> = HashMap::new();
            for row_idx in 0..bingo::ROWS {
                let row = &cells[row_idx];
                for col_idx in 0..bingo::COLS {
                    let coord = (row_idx, col_idx);
                    let col = &row[col_idx];
                    let vec_of_coords = values_to_coords
                        .entry(col.value)
                        .or_insert(Vec::<(usize, usize)>::new());
                    vec_of_coords.push(coord);
                }
            }

            return values_to_coords;
        }

        /// Given a sequence of numbers, determine:
        ///  1. Whether the board will hit Bingo!, and if so
        ///  2. Which number (by index/position) triggered the bingo, and
        ///  3. Which board cells were hit & not hit (by coordinate, row/col)
        pub fn predict(&self, numbers: &Vec<u32>) -> Option<Prediction> {  // bingo

            let mut cells: [[Cell; bingo::COLS]; bingo::ROWS] = self.grid.map(
                |row| {
                    row.map(
                        |col| {
                            Cell { value: col, hit: false, }
                        }
                    )
                }
            );

            let values_to_coords = self.index(&cells);

            // For each number, check if a cell hits and if that lead to bingo.
            for number_idx in 0..numbers.len() {
                let number = numbers[number_idx];

                // Do any cells have a value matching the number?
                let vec_of_coords = match values_to_coords.get(&number) {
                    Some(vec_of_coords) => vec_of_coords,
                    None => continue,
                };

                // Mark matching cells as hit and check for Bingo!
                for coord in vec_of_coords {
                    cells[coord.0][coord.1].hit = true;
                    if self.check(&cells, &coord) {
                        return Some(Prediction {
                            when: number_idx,
                            score: self.score(&cells, number),
                        })
                    }
                }

            }

            return None;
        }

    }

}

mod part1 {

    use super::*;

    /// Given a sequence of strings like this:
    /// ```
    /// let seq = vec![
    ///    "22 13 17 11  0
    ///      8  2 23  4 24
    ///     21  9 14 16  7
    ///      6 10  3 18  5
    ///     1 12 20 15 19",
    /// ];
    /// ```
    /// Create column-major grids and store them in a bingo::Board.
    fn boards(source: &[&str]) -> Vec<bingo::Board> {

        // Get a sequence matrices that are hopefully 5x5 Bingo boards.
        let boards_from_strs: Vec<Vec<Vec<u32>>> = source.iter().map(
            |block| {
                block.split('\n').map(
                    |line| {
                        line.split_ascii_whitespace().map(
                            |n| { n.parse().unwrap() }
                        ).collect()
                    }
                ).collect()
            }
        ).collect();

        // Convert each matrix parsed from the input into a fixed array of ints
        // (which have performance benefits from their const size).
        let mut grids: Vec<[[u32; bingo::COLS]; bingo::ROWS]> = Vec::new();
        for board_from_str in boards_from_strs.iter() {
            let mut grid: [[u32; bingo::COLS]; bingo::ROWS] = [[0; bingo::COLS]; bingo::ROWS];
            for row_idx in 0..bingo::ROWS {
                for col_idx in 0..bingo::COLS {
                    grid[row_idx][col_idx] = board_from_str[row_idx][col_idx];
                }
            }
            grids.push(grid);
        }

        // Create the bingo::Boards
        let boards: Vec<bingo::Board> = grids.into_iter().map(
            |grid| {
                bingo::Board {
                    grid: grid,
                }
            }
        ).collect();

        return boards;
    }

    /// Parse the part1 input (which might be reused in part2) into the sequence
    /// of "numbers" to be played and a collection of Bingo "boards".
    pub fn input(source: &str) -> bingo::Input {
        let lines: Vec<&str> = source.split("\n\n").collect();
        let numbers: Vec<u32> = lines[0].split(',').map( |n| { n.trim().parse().unwrap() }).collect();
        let boards: Vec<bingo::Board> = boards(&lines[1..]);
        return bingo::Input {
            numbers: numbers,
            boards: boards,
        }
    }

    /// Given the numbers and boards to be played at Bingo, determine when each
    /// board will win and with what score (if it ever wins). Return the score
    /// for the Bingo board that will win the soonest.
    pub fn solution(input: &bingo::Input) -> u32 {

        // Running the numbers over each board will return a "prediction" with
        // the win time and score. Filter out losing boards (None) and unpack
        // the Some(prediction) to get a sequence of Predictions.
        let predictions = &mut input.boards.iter()
            .map( |board| { board.predict(&input.numbers) } )
            .filter( |prediction| prediction.is_some() )
            .map( |prediction| prediction.unwrap() )
            .collect::<Vec<bingo::Prediction>>();

        // Sort the results in ascending order of when they'll win the game.
        predictions.sort_by(|a, b| b.score.cmp(&a.score) );

        // Return the score of the board that will win the soonest.
        return predictions[0].score;

    }

}


pub fn main(input_path: &Path) {

    let part1_input_source: String = fs::read_to_string(input_path).expect("failed to read part 1 input");
    let part1_input = part1::input(&part1_input_source);
    let part1_output = part1::solution(&part1_input);
    println!("Part 1 answer: {}", part1_output);

}


#[cfg(test)]
mod tests {
    use super::*;
    use super::super::utils;

    /// The example inputs and outputs given by AOC
    #[test]
    fn test_part1_example() {

        let example_input = "
        7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

        22 13 17 11  0
         8  2 23  4 24
        21  9 14 16  7
         6 10  3 18  5
         1 12 20 15 19

         3 15  0  2 22
         9 18 13 17  5
        19  8  7 25 23
        20 11 10 24  4
        14 21 16 12  6

        14 21 17 24  4
        10 16 15  9 19
        18  8 23 26 20
        22 11 13  6  5
         2  0 12  3  7
        ";
        let example_input_cleaned = utils::remove_multiline_whitespace(&example_input);
        let part1_input = part1::input(&example_input_cleaned);
        let part1_output = part1::solution(&part1_input);
        assert_eq!(part1_output, 4512);

    }

}