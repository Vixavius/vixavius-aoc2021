// ------------------------------------------------------------------------------------------------
// Advent of Code 2021 - Day 8 - Part 1
// ------------------------------------------------------------------------------------------------
// https://adventofcode.com/2021/day/8
// ------------------------------------------------------------------------------------------------
// mvr@michaelvanryn.com
// December 8, 2021
// ------------------------------------------------------------------------------------------------
//
//  AAAA   The signal connection labels
// B    C
// B    C
//  DDDD
// E    F
// E    F
//  GGGG
//
// ------------------------------------------------------------------------------------------------

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Pattern {
    //signals: Vec<String>,
    outputs: Vec<String>,
}

pub fn run() -> i128 {
    // Vector of signal patterns and outputs
    let mut patterns: Vec<Pattern> = vec![];

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input/day08.txt") {
        // Parse each line as string from text file, force unwrapping Result
        for line in lines {
            if let Ok(ip) = line {
                let split: Vec<&str> = ip.split(" | ").collect();

                if split.len() == 2 {
                    //let signals: Vec<String> = split[0].split_whitespace().map(|s| s.to_string()).collect();
                    let outputs: Vec<String> =
                        split[1].split_whitespace().map(|s| s.to_string()).collect();

                    patterns.push(Pattern {
                        /* signals: signals, */ outputs: outputs,
                    });
                }
            }
        }
    } else {
        println!("Failed to open file");
    }

    let mut sum: i128 = 0;

    for pattern in patterns {
        for output in pattern.outputs {
            if output.len() == 2 || output.len() == 3 || output.len() == 4 || output.len() == 7 {
                sum += 1;
            }
        }
    }

    return sum;
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
