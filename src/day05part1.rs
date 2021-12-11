// ------------------------------------------------------------------------------------------------
// Advent of Code 2021 - Day 5 - Part 1
// ------------------------------------------------------------------------------------------------
// https://adventofcode.com/2021/day/5
// ------------------------------------------------------------------------------------------------
// mvr@michaelvanryn.com
// December 5, 2021
// ------------------------------------------------------------------------------------------------
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct LineSegment {
    x1: u32,
    x2: u32,
    y1: u32,
    y2: u32,
}

pub fn run() -> i128 {
    // Vector of bingo boards
    let mut linesegments: Vec<LineSegment> = vec![];
    let mut grid = [[0u8; 1000]; 1000];

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input/day05.txt") {
        // Parse each line as string from text file, force unwrapping Result

        for line in lines {
            if let Ok(ip) = line {
                let split: Vec<&str> = ip.split(" -> ").collect();

                if split.len() == 2 {
                    let left: Vec<&str> = split[0].split(",").collect();
                    let right: Vec<&str> = split[1].split(",").collect();

                    if left.len() == 2 && right.len() == 2 {
                        linesegments.push(LineSegment {
                            x1: left[0].parse::<u32>().unwrap(),
                            x2: right[0].parse::<u32>().unwrap(),
                            y1: left[1].parse::<u32>().unwrap(),
                            y2: right[1].parse::<u32>().unwrap(),
                        });
                    }
                }
            }
        }
    } else {
        println!("Failed to open file");
    }

    for segment in linesegments {
        // Only consider horizontal or vertical lines
        if segment.y1 == segment.y2 {
            for i in std::cmp::min(segment.x1, segment.x2)..=std::cmp::max(segment.x1, segment.x2) {
                grid[i as usize][segment.y1 as usize] += 1;
            }
        } else if segment.x1 == segment.x2 {
            for i in std::cmp::min(segment.y1, segment.y2)..=std::cmp::max(segment.y1, segment.y2) {
                grid[segment.x1 as usize][i as usize] += 1;
            }
        }
    }

    // Calculate the # of points overlapping
    let mut points_overlapping: i128 = 0;

    for i in 0..1000 {
        for j in 0..1000 {
            if grid[i][j] > 1 {
                points_overlapping += 1;
            }
        }
    }

    return points_overlapping;
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
