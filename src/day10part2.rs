// ------------------------------------------------------------------------------------------------
// Advent of Code 2021 - Day 10 - Part 2
// ------------------------------------------------------------------------------------------------
// https://adventofcode.com/2021/day/10
// ------------------------------------------------------------------------------------------------
// mvr@michaelvanryn.com
// December 11, 2021
// ------------------------------------------------------------------------------------------------
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(PartialEq)]
enum SequenceValidity {
    Unknown,
    Corrupt,
}

struct Sequence {
    pattern: Vec<char>,
    state: SequenceValidity,
}

pub fn run() -> i128 {
    // 2D vector representing command lines
    let mut patterns: Vec<Sequence> = vec![];

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input/day10.txt") {
        // Parse each line as string from text file, force unwrapping Result
        for line in lines {
            if let Ok(ip) = line {
                let mut lines: Vec<char> = vec![];

                for c in ip.chars() {
                    lines.push(c);
                }

                patterns.push(Sequence {
                    pattern: lines.clone(),
                    state: SequenceValidity::Unknown,
                });
            }
        }
    } else {
        println!("Failed to open file");
    }

    let mut scores: Vec<u64> = vec![];

    for line in patterns.iter_mut() {
        let mut stack: Vec<char> = vec![];

        for (i, c) in line.pattern.iter().enumerate() {
            if line.state != SequenceValidity::Unknown {
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
                if *c == ')' || *c == ']' || *c == '}' || *c == '>' {
                    if *c == expected {
                        stack.pop();
                    } else {
                        line.state = SequenceValidity::Corrupt;
                    }
                }
            }

            // At the last index, if the sequence is not corrupt, it must be incomplete
            if i == line.pattern.len() - 1 && line.state == SequenceValidity::Unknown {
                let mut score: u64 = 0;

                // Determine the closing characters required to complete the sequence
                for k in stack.iter().rev() {
                    score *= 5;
                    match *k {
                        '(' => score += 1,
                        '[' => score += 2,
                        '{' => score += 3,
                        '<' => score += 4,
                        _ => (),
                    }
                }

                scores.push(score);
            }
        }
    }

    scores.sort();

    return scores[scores.len() / 2] as i128;
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
