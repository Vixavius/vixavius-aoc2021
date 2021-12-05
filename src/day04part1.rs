// ------------------------------------------------------------------------------------------------
// Advent of Code 2021 - Day 4 - Part 1
// ------------------------------------------------------------------------------------------------
// https://adventofcode.com/2021/day/4
// ------------------------------------------------------------------------------------------------
// mvr@michaelvanryn.com
// December 4, 2021
// ------------------------------------------------------------------------------------------------
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct BingoNumber {
    value: u32,
    marked: bool,
}

struct BingoBoard {
    board: Vec<Vec<BingoNumber>>,
}

impl BingoBoard {
    fn check_for_victory(&self) -> bool {
        for row in &self.board {
            let mut win = true;
            
            for col in row {
                win &= col.marked;
            }
    
            if win {
                return true;
            }
        }

        if self.board.len() > 0 && self.board[0].len() > 0 {
            for i in 0..self.board.len() {
                let mut win = true;

                for j in 0..self.board[0].len() {
                    win &= self.board[j][i].marked;
                }

                if win {
                    return true;
                }
            }
        }
        
        return false;
    }

    fn calculate_score(&self) -> u32 {
        let mut score: u32 = 0;

        for row in &self.board {
            for col in row {
                if !col.marked {
                    score += col.value;
                }
            }
        }

        return score;
    }
}

pub fn run() -> i128 {
    // Vector of bingo boards 
    let mut numbers: Vec<u32> = vec![];
    let mut boards: Vec<BingoBoard> = vec![];

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input/day04.txt") {
        // Parse each line as string from text file, force unwrapping Result
        let mut new_board = BingoBoard{board: vec![vec![]]};
        new_board.board.clear();

        for (i, line) in lines.enumerate() {
            if let Ok(ip) = line {
                if i == 0 {
                     let split: Vec<&str> = ip.split(',').collect();

                     for number in split {
                         numbers.push(number.parse::<u32>().unwrap());
                     }
                } else {
                    let board_row: Vec<u32> = ip.split_whitespace().map(|s| s.parse().expect("parse error")).collect();

                    if board_row.len() > 0 {
                        let mut bingo_board_row: Vec<BingoNumber>  = vec![];

                        for number in &board_row {
                            bingo_board_row.push(BingoNumber{value: *number, marked: false});
                        }

                        new_board.board.push(bingo_board_row);

                        if new_board.board.len() == 5 {
                            boards.push(new_board);
                            new_board = BingoBoard{board: vec![vec![]]};
                            new_board.board.clear();
                        }
                    }
                }
            }
        }
    } else {
        println!("Failed to open file");
    }

    // Mark each number as winning, and check for victories
    let mut victory: i32 = -1;

    for number in numbers {
        for board in boards.iter_mut() {
            for (_i, row) in board.board.iter_mut().enumerate() {
                for (_j, col) in row.iter_mut().enumerate() {
                    if number == col.value {
                        col.marked = true;
                    }
                }
            }
        }

        for (i, board) in boards.iter().enumerate() {
            if board.check_for_victory() {
                victory = i as i32;
            }
        }

        if victory != -1 {//&& victory >= boards.len() as i32 {
            let sum = boards[victory as usize].calculate_score();
            return (sum * number) as i128;
        }
    }


    return 0;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}