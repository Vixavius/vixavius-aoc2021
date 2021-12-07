// ------------------------------------------------------------------------------------------------
// Advent of Code 2021 - Day 6 - Part 1
// ------------------------------------------------------------------------------------------------
// https://adventofcode.com/2021/day/1
// ------------------------------------------------------------------------------------------------
// mvr@michaelvanryn.com
// December 6, 2021
// ------------------------------------------------------------------------------------------------
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Lanternfish {
    timer: u32,
}

impl Lanternfish {
    // Returns true if a new lanternfish should be born
    fn update(&mut self) -> bool {
        if self.timer == 0 {
            self.timer = 6;

            return true;
        }

        self.timer -= 1;

        return false;
    }
}

pub fn run() -> i128 {
    // Vector of bingo boards 
    let mut fishes:[u64;9] = [0; 9];

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input/day06.txt") {
        // Parse each line as string from text file, force unwrapping Result
        for line in lines {
            if let Ok(ip) = line {
                let split: Vec<&str> = ip.split(",").collect();

                for i in split {
                    let number = i.parse::<u32>().unwrap();

                    if number < 9 {
                        fishes[number as usize] += 1;
                    }
                }
            }
        }
    } else {
        println!("Failed to open file");
    }

    for fish in fishes {
        print!("{},", fish);

    }
    println!("");
    
    for day in 1..=80 {
        let fish_to_add = fishes[0];

        // Shift the array to the left
        for i in 0..(fishes.len() - 1) {
            fishes[i] = fishes[i+1];
        }

        fishes[6] += fish_to_add; 
        fishes[8] = fish_to_add;
    

        for fish in fishes {
            print!("{},", fish);

        }
        println!("");

        let mut sum = 0;

        for fish in fishes {
            sum += fish;
        }
        println!("after day {} # fish {}", day, sum);
    }

    // Sum the fishes
    let mut sum = 0;

    for fish in fishes {
        sum += fish;
    }

    return sum as i128;
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