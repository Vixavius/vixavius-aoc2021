// ------------------------------------------------------------------------------------------------
// Advent of Code 2021 - Day 10 - Part 1
// ------------------------------------------------------------------------------------------------
// https://adventofcode.com/2021/day/10
// ------------------------------------------------------------------------------------------------
// mvr@michaelvanryn.com
// December 11, 2021
// ------------------------------------------------------------------------------------------------
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() -> i128 {
    // 2D vector representing command lines
    let mut patterns: Vec<Vec<char>> = vec![vec![]];
    patterns.clear();

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input/day10.txt") {
        // Parse each line as string from text file, force unwrapping Result
        for line in lines {
            if let Ok(ip) = line {
                let mut lines: Vec<char> = vec![];

                for c in ip.chars() {
                    lines.push(c);
                }

                patterns.push(lines);
            }
        }
    } else {
        println!("Failed to open file");
    }

    let mut error_score: i128 = 0;

    for (_i, line) in patterns.iter().enumerate() {
        let mut stack: Vec<char> = vec![];
        let mut valid = true;

        for c in line {
            if !valid {
                break;
            }

            let mut expected = '_';

            if stack.len() > 0 {
                match stack.last().unwrap() {
                    '(' => expected = ')',
                    '[' => expected = ']',
                    '{' => expected = '}',
                    '<' => expected = '>',
                    _ => (),
                };
            }

            if *c == '(' || *c == '[' || *c == '{' || *c == '<' {
                stack.push(*c)
            } else if stack.len() > 0 {
                if *c == ')' {
                    if *c == expected {
                        stack.pop();
                    } else {
                        valid = false;
                        error_score += 3;
                    }
                } else if *c == ']' {
                    if *c == expected {
                        stack.pop();
                    } else {
                        valid = false;
                        error_score += 57;
                    }
                } else if *c == '}' {
                    if *c == expected {
                        stack.pop(); 
                    } else {
                        valid = false;
                        error_score += 1197;
                    }
                } else if *c == '>' {
                    if *c == expected {
                        stack.pop();
                    } else {
                        valid = false;
                        error_score += 25137;
                    }
                }
            }
        }
    }

    return error_score;
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
