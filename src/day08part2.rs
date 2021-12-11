// ------------------------------------------------------------------------------------------------
// Advent of Code 2021 - Day 8 - Part 2
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

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Pattern {
    signals: Vec<String>,
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
                    let signals: Vec<String> =
                        split[0].split_whitespace().map(|s| s.to_string()).collect();
                    let outputs: Vec<String> =
                        split[1].split_whitespace().map(|s| s.to_string()).collect();

                    patterns.push(Pattern {
                        signals: signals,
                        outputs: outputs,
                    });
                }
            }
        }
    } else {
        println!("Failed to open file");
    }

    let mut sum: i128 = 0;

    for pattern in patterns {
        let mut formed: HashMap<u8, String> = HashMap::new();
        let mut remaining: Vec<String> = pattern.signals.clone();

        // Find all the unique signal pattern lengths
        for (_i, signal) in pattern.signals.iter().enumerate() {
            let mut found = false;

            match signal.len() {
                2 => {
                    formed.insert(1, String::from(signal));
                    found = true
                }
                3 => {
                    formed.insert(7, String::from(signal));
                    found = true
                }
                4 => {
                    formed.insert(4, String::from(signal));
                    found = true
                }
                7 => {
                    formed.insert(8, String::from(signal));
                    found = true
                }
                _ => (),
            };

            if found {
                remaining.retain(|x| x != signal);
            }
        }

        // Signal pattern 3 -- length of 5 with 7 segments being a subset
        for (_i, signal) in pattern.signals.iter().enumerate() {
            if remaining.contains(signal)
                && signal.len() == 5
                && is_subset(signal, formed.get(&7).unwrap())
            {
                formed.insert(3, String::from(signal));
                remaining.retain(|x| x != signal);
                break;
            }
        }

        // Signal pattern 9 -- length of 6 with 7 and 4 segments being subsets
        for (_i, signal) in pattern.signals.iter().enumerate() {
            if remaining.contains(signal)
                && signal.len() == 6
                && is_subset(signal, formed.get(&7).unwrap())
                && is_subset(signal, formed.get(&4).unwrap())
            {
                formed.insert(9, String::from(signal));
                remaining.retain(|x| x != signal);
                break;
            }
        }

        // Signal pattern 6 -- length of 6 that is not a subset of 7
        for (_i, signal) in pattern.signals.iter().enumerate() {
            if remaining.contains(signal)
                && signal.len() == 6
                && !is_subset(signal, formed.get(&7).unwrap())
            {
                formed.insert(6, String::from(signal));
                remaining.retain(|x| x != signal);
                break;
            }
        }

        // Signal pattern 0 -- length of 6 remaining
        for (_i, signal) in pattern.signals.iter().enumerate() {
            if remaining.contains(signal) && signal.len() == 6 {
                formed.insert(0, String::from(signal));
                remaining.retain(|x| x != signal);
                break;
            }
        }

        // Signal pattern 5 -- length of 5 and subset of 9
        for (_i, signal) in pattern.signals.iter().enumerate() {
            if remaining.contains(signal)
                && signal.len() == 5
                && is_subset(formed.get(&9).unwrap(), signal)
            {
                formed.insert(5, String::from(signal));
                remaining.retain(|x| x != signal);
                break;
            }
        }

        // Signal pattern 2 -- length of 5 and not a subset of 9
        for (_i, signal) in pattern.signals.iter().enumerate() {
            if remaining.contains(signal) && signal.len() == 5 {
                formed.insert(2, String::from(signal));
                remaining.retain(|x| x != signal);
                break;
            }
        }

        // Determine the output values
        let mut output_value: i128 = 0;

        for (i, output) in pattern.outputs.iter().enumerate() {
            // Determine the output value
            let multiplier: i128 = match i {
                0 => 1000,
                1 => 100,
                2 => 10,
                3 => 1,
                _ => 0,
            };

            for (key, value) in &formed {
                if value.len() == output.len() && is_subset(value, output) {
                    output_value += *key as i128 * multiplier;
                }
            }
        }

        sum += output_value;
    }

    return sum;
}

// Determine if a String is a subset of another String
fn is_subset(set: &String, subset: &String) -> bool {
    for c in subset.chars() {
        if !set.contains(&String::from(c)) {
            return false;
        }
    }

    return true;
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
