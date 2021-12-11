// ------------------------------------------------------------------------------------------------
// Advent of Code 2021 - Day 9 - Part 1
// ------------------------------------------------------------------------------------------------
// https://adventofcode.com/2021/day/9
// ------------------------------------------------------------------------------------------------
// mvr@michaelvanryn.com
// December 11, 2021
// ------------------------------------------------------------------------------------------------
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() -> i128 {
    // Map of the lava tubes
    let mut map: Vec<Vec<u32>> = vec![vec![]];
    map.clear();

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input/day09.txt") {
        // Parse each line as string from text file, force unwrapping Result
        for line in lines {
            if let Ok(ip) = line {
                let mut row: Vec<u32> = vec![];

                for c in ip.chars().into_iter() {
                    row.push(c.to_digit(10).unwrap());
                }

                map.push(row);
            }
        }
    } else {
        println!("Failed to open file");
    }

    let mut risks: Vec<u32> = vec![];

    for (i, row) in map.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            let mut lowest: u32 = 9;

            if i > 0 && map[i - 1][j] < lowest {
                lowest = map[i - 1][j];
            }
            if j > 0 && map[i][j - 1] < lowest {
                lowest = map[i][j - 1];
            }
            if j < row.len() - 1 && map[i][j + 1] < lowest {
                lowest = map[i][j + 1];
            }
            if i < map.len() - 1 && map[i + 1][j] < lowest {
                lowest = map[i + 1][j];
            }

            if *cell < lowest {
                risks.push(*cell + 1);
            }
        }
    }

    let mut sum: i128 = 0;

    for risk in risks {
        sum += risk as i128;
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
