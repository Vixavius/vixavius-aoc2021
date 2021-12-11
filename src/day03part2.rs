// ------------------------------------------------------------------------------------------------
// Advent of Code 2021 - Day 3 - Part 2
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
    let mut numbers: Vec<String> = vec![];

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input/day03.txt") {
        // Parse each line as string from text file, force unwrapping Result
        for line in lines {
            if let Ok(ip) = line {
                numbers.push(ip);
            }
        }
    } else {
        println!("Failed to open file");
    }

    // Oxygen Generator Rating - 1
    let mut oxygen_numbers = numbers.clone();
    let mut i: usize = 0;

    while i < numbers[0].len() && oxygen_numbers.len() > 1 {
        let count = count_digits(&oxygen_numbers, i);
        let mut c = '0';

        if count >= 0 {
            c = '1'
        }

        oxygen_numbers.retain(|x| x.to_string().chars().nth(i).unwrap() == c);

        i += 1;
    }

    // CO2 Scrubber Rating - 0
    let mut co2_numbers = numbers.clone();
    i = 0;

    while i < numbers[0].len() && co2_numbers.len() > 1 {
        let count = count_digits(&co2_numbers, i);
        let mut c = '0';

        if count < 0 {
            c = '1'
        }

        co2_numbers.retain(|x| x.to_string().chars().nth(i).unwrap() == c);
        i += 1;
    }

    if oxygen_numbers.len() > 0 && co2_numbers.len() > 0 {
        let oxygen_rating = isize::from_str_radix(oxygen_numbers[0].as_str(), 2).unwrap();
        let scrubber_rating = isize::from_str_radix(co2_numbers[0].as_str(), 2).unwrap();

        return (oxygen_rating * scrubber_rating) as i128;
    }

    return 0;
}

fn count_digits(numbers: &Vec<String>, i: usize) -> i32 {
    let mut count: i32 = 0;

    for n in numbers {
        if i < n.len() {
            if n.to_string().chars().nth(i).unwrap() == '0' {
                count -= 1;
            } else {
                count += 1;
            }
        }
    }

    return count;
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
