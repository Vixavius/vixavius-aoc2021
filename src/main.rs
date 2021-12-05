mod day01part1;
mod day01part2;
mod day02part1;
mod day02part2;
mod day03part1;
mod day03part2;
mod day04part1;
mod day04part2;
mod day05part1;
mod day05part2;

use pad::PadStr;
use std::time::{Duration, Instant};

const NUMBER_OF_RUNS: u64 = 1;

pub fn main() {
    let initial = Instant::now();
    let mut start = Instant::now();
    let current = start.elapsed();

    let problems: Vec<&dyn Fn() -> i128> = vec![
        &day01part1::run,
        &day01part2::run,
        &day02part1::run,
        &day02part2::run,
        &day03part1::run,
        &day03part2::run,
        &day04part1::run,
        &day04part2::run,
        &day05part1::run,
        &day05part2::run,
    ];
    let mut avg_runtimes: Vec<f64> = vec![];
    let mut outputs: Vec<String> = vec![];

    for (i, problem) in problems.iter().enumerate() {
        let mut runtimes: Vec<f64> = vec![];
        let mut result: i128 = 0;

        for _i in 0..NUMBER_OF_RUNS {
            start = Instant::now();
            result = problem();
            runtimes.push(runtime(start, &current));
        }

        avg_runtimes.push(format(average(runtimes)));

        let mut output: String = String::from("Day ") + &(i / 2 + 1).to_string();
        output = output.pad_to_width(7);

        if i % 2 == 0 {
            output += " Part 1";
        } else {
            output += " Part 2";
        }

        output += " - ";
        output += &result.to_string().pad_to_width(17);
        output += " - ";
        output += &avg_runtimes.last().unwrap().to_string().pad_to_width(6);
        output += " s ";
        outputs.push(output);
    }

    println!("\n\n-------------------------------------------------");
    println!("      A D V E N T   O F   C O D E   2 0 2 1 ");
    println!("-------------------------------------------------\n");

    for output in outputs {
        println!("{}", output);
    }

    println!(
        "Total Runtime ({} Passes): {} s\n\n\n",
        NUMBER_OF_RUNS,
        format(runtime(initial, &current))
    );
}

fn runtime(start: Instant, current: &Duration) -> f64 {
    return start.elapsed().as_secs_f64() as f64 - current.as_secs_f64() as f64;
}

fn format(number: f64) -> f64 {
    return (number * 10000f64).round() / 10000f64;
}

fn average(numbers: Vec<f64>) -> f64 {
    numbers.iter().sum::<f64>() / numbers.len() as f64
}
