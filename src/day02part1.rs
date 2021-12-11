// ------------------------------------------------------------------------------------------------
// Advent of Code 2021 - Day 2 - Part 1
// ------------------------------------------------------------------------------------------------
// https://adventofcode.com/2021/day/2
// ------------------------------------------------------------------------------------------------
// mvr@michaelvanryn.com
// December 2, 2021
// ------------------------------------------------------------------------------------------------
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Forward,
    None,
}

struct DiveCommand {
    direction: Direction,
    value: u32,
}

pub fn run() -> i128 {
    // Vector of Dive Commands
    let mut values: Vec<DiveCommand> = vec![];

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input/day02.txt") {
        // Parse each line as string from text file, force unwrapping Result
        for line in lines {
            if let Ok(ip) = line {
                let splits: Vec<&str> = ip.split(" ").collect();

                if splits.len() == 2 {
                    let direction = match splits[0] {
                        "up" => Direction::Up,
                        "down" => Direction::Down,
                        "forward" => Direction::Forward,
                        _ => Direction::None,
                    };

                    values.push(DiveCommand {
                        direction: direction,
                        value: splits[1].parse::<u32>().unwrap(),
                    });
                    //println!("{:?} {}", values[values.len() - 1].direction, values[values.len() - 1].value);
                }
            }
        }
    } else {
        println!("Failed to open file");
    }

    let mut position: i32 = 0;
    let mut depth: i32 = 0;

    for command in values {
        match command.direction {
            Direction::Up => depth -= command.value as i32,
            Direction::Down => depth += command.value as i32,
            Direction::Forward => position += command.value as i32,
            _ => {}
        }
    }

    //println!("Final position: {}, and final depth: {}", position, depth);

    return (position * depth) as i128;
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
