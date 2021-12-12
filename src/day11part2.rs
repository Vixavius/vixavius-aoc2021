// ------------------------------------------------------------------------------------------------
// Advent of Code 2021 - Day 11 - Part 1
// ------------------------------------------------------------------------------------------------
// https://adventofcode.com/2021/day/11
// ------------------------------------------------------------------------------------------------
// mvr@michaelvanryn.com
// December 11, 2021
// ------------------------------------------------------------------------------------------------
use core::cmp::max;
use core::cmp::min;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct DumboOctopus {
    energy: u32,
    flashed: bool,
}

impl DumboOctopus {
    fn update(&mut self) -> bool {
        if self.flashed {
            self.energy = 0;
            self.flashed = false;

            return true;
        }

        return false;
    }

    fn increment(&mut self) -> bool {
        self.energy += 1;

        if self.energy == 10 {
            self.flashed = true;
        }

        return self.energy == 10;
    }
}

pub fn run() -> i128 {
    // 2D vector representing Dumbo Octopodes
    let mut octopodes: Vec<Vec<DumboOctopus>> = vec![vec![]];
    octopodes.clear();

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input/day11.txt") {
        // Parse each line as string from text file, force unwrapping Result
        for line in lines {
            if let Ok(ip) = line {
                let mut line: Vec<DumboOctopus> = vec![];

                for c in ip.chars() {
                    line.push(DumboOctopus {
                        energy: c.to_digit(10).unwrap(),
                        flashed: false,
                    });
                }

                octopodes.push(line);
            }
        }
    } else {
        println!("Failed to open file");
    }

    let mut flashing_octopodes: Vec<(usize, usize)> = vec![];
    let mut unflashed: u32 = 1;
    let mut step: u32 = 0;

    while unflashed != 0 {
        step += 1;
        unflashed = 0;

        // Step 1 - Increment all energy counters by 1
        for (j, row) in octopodes.iter_mut().enumerate() {
            for (k, octopus) in row.iter_mut().enumerate() {
                if octopus.increment() {
                    flashing_octopodes.push((j, k));
                }
            }
        }

        // Step 2 - Flash all octopodes until there are no remaining flashers
        while flashing_octopodes.len() > 0 {
            let flasher = flashing_octopodes.pop().unwrap();

            let row_l = max(0, flasher.1 as i32 - 1);
            let row_r = min(octopodes[0].len() as i32 - 1, flasher.1 as i32 + 1);
            let col_u = max(0, flasher.0 as i32 - 1);
            let col_d = min(octopodes.len() as i32 - 1, flasher.0 as i32 + 1);

            for y in col_u..=col_d {
                for x in row_l..=row_r {
                    if flasher.0 as i32 != y || flasher.1 as i32 != x {
                        //println!("Incrementing {} {}", x, y);
                        if octopodes[y as usize][x as usize].increment() {
                            flashing_octopodes.push((y as usize, x as usize));
                        }
                    }
                }
            }
        }

        // Step 3 - Update the flashed octopodes
        for (_j, row) in octopodes.iter_mut().enumerate() {
            for (_k, octopus) in row.iter_mut().enumerate() {
                if !octopus.update() {
                    unflashed += 1;
                };
            }
        }
    }

    return step as i128;
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
