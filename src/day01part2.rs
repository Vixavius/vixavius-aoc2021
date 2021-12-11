// ------------------------------------------------------------------------------------------------
// Advent of Code 2021 - Day 1 - Part 2
// ------------------------------------------------------------------------------------------------
// https://adventofcode.com/2021/day/1
// ------------------------------------------------------------------------------------------------
// mvr@michaelvanryn.com
// December 1, 2021
// ------------------------------------------------------------------------------------------------
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() -> i128 {
    let mut values = vec![];

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input/day01.txt") {
        // Parse each line as integer from text file, force unwrapping Result
        for line in lines {
            if let Ok(ip) = line {
                values.push(ip.parse::<u32>().unwrap());
            }
        }
    } else {
        println!("Failed to open file");
    }

    let mut n: u32 = 0;

    for (i, &x) in values.iter().enumerate() {
        if i >= 3 {
            if x > values[i - 3] {
                n += 1;
            }
        }
    }

    return n as i128;
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
