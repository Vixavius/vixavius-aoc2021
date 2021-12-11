// ------------------------------------------------------------------------------------------------
// Advent of Code 2021 - Day 7 - Part 1
// ------------------------------------------------------------------------------------------------
// https://adventofcode.com/2021/day/7
// ------------------------------------------------------------------------------------------------
// mvr@michaelvanryn.com
// December 7, 2021
// ------------------------------------------------------------------------------------------------
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() -> i128 {
    let mut positions: Vec<u64> = vec![];

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input/day07.txt") {
        // Parse each line as string from text file, force unwrapping Result
        for line in lines {
            if let Ok(ip) = line {
                let split: Vec<&str> = ip.split(",").collect();

                // Input parsing
                for number in split {
                    positions.push(number.parse::<u64>().unwrap());
                }
            }
        }
    } else {
        println!("Failed to open file");
    }

    // Brute force approach
    let mut fuels: Vec<u64> = vec![];
    let mut fuel;
    let mut max: u64 = 0;
    let mut cheapest: u64 = std::u64::MAX;

    for position in positions.iter() {
        if position > &max {
            max = *position;
        }
    }

    // For each possible position
    for i in 0..=max {
        fuel = 0;

        // Sum all required fuel
        for position in positions.iter() {
            fuel += i64::abs(*position as i64 - i as i64) as u64;
        }

        fuels.push(fuel);
    }

    // Find the smallest fuel used
    for fuel in fuels {
        if fuel < cheapest {
            cheapest = fuel;
        }
    }

    return cheapest as i128;
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
