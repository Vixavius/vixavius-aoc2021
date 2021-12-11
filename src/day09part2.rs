// ------------------------------------------------------------------------------------------------
// Advent of Code 2021 - Day 9 - Part 2
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
    let mut basin_map: Vec<Vec<bool>> = vec![vec![]];

    map.clear();
    basin_map.clear();

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

    for row in &map {
        let mut basin_row: Vec<bool> = vec![];

        for _cell in row {
            basin_row.push(false);
        }

        basin_map.push(basin_row);
    }

    // Find the low points
    let mut low_points: Vec<(usize, usize)> = vec![];

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
                low_points.push((i, j));
            }
        }
    }

    let mut basin_sizes: Vec<u32> = vec![];

    for low_point in low_points {
        basin_sizes.push(mark_neighbours(
            &map,
            &mut basin_map,
            low_point.0,
            low_point.1,
        ));
    }

    // Find the 3 largest basins
    let mut first: u32 = 0;
    let mut second: u32 = 0;
    let mut third: u32 = 0;

    for (_i, basin) in basin_sizes.iter().enumerate() {
        if *basin > first {
            third = second;
            second = first;
            first = *basin;
        } else if *basin > second {
            third = second;
            second = *basin;
        } else if *basin > third {
            third = *basin;
        }
    }

    return (first * second * third) as i128;
}

fn mark_neighbours(map: &Vec<Vec<u32>>, basin: &mut Vec<Vec<bool>>, y: usize, x: usize) -> u32 {
    let mut neighbours: u32 = 0;

    if y < map.len() && x < map[y].len() && map[y][x] == 9 {
        return 0;
    }

    basin[y][x] = true;

    if y > 0 && map[y - 1][x] > map[y][x] && !basin[y - 1][x] {
        neighbours += mark_neighbours(map, basin, y - 1, x);
    }
    if x > 0 && map[y][x - 1] > map[y][x] && !basin[y][x - 1] {
        neighbours += mark_neighbours(map, basin, y, x - 1);
    }
    if x < map[y].len() - 1 && map[y][x + 1] > map[y][x] && !basin[y][x + 1] {
        neighbours += mark_neighbours(map, basin, y, x + 1);
    }
    if y < map.len() - 1 && map[y + 1][x] > map[y][x] && !basin[y + 1][x] {
        neighbours += mark_neighbours(map, basin, y + 1, x);
    }

    return neighbours + 1;
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
