// ------------------------------------------------------------------------------------------------
// Advent of Code 2021 - Day 3 - Part 1
// ------------------------------------------------------------------------------------------------
// https://adventofcode.com/2021/day/3
// ------------------------------------------------------------------------------------------------
// mvr@michaelvanryn.com
// December 3, 2021
// ------------------------------------------------------------------------------------------------
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() -> i128 {
    // Vector of Dive Commands
    let mut digits: Vec<i32> = vec![];

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input/day03.txt") {
        // Parse each line as string from text file, force unwrapping Result
        for line in lines {
            if let Ok(ip) = line {
                for (i, c) in ip.chars().enumerate() {
                    if i as i32 > digits.len() as i32 - 1 {
                        digits.push(0);
                    }
                    if c == '1' {
                        digits[i] += 1;
                    } else {
                        digits[i] -= 1;
                    }
                }
            }
        }
    } else {
        println!("Failed to open file");
    }

    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;

    for digit in digits {
        gamma = gamma << 1;
        epsilon = epsilon << 1;
        if digit > 0 {
            gamma += 1;
        } else if digit < 0 {
            epsilon += 1;
        }

    }

    return (gamma * epsilon) as i128;
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
