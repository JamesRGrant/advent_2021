// Advent of Code 2021: https://adventofcode.com/2021 (Google Auth)
mod solutions;
use std::io::{BufRead, BufReader};
const TEST_MODE: bool = true;

trait Solve {
    fn p1(&mut self) -> i64;
    fn p2(&mut self) -> i64;
}

fn main() {
    let start = std::time::Instant::now();
    for day in 1..=25 {
        // Assumes &data files for each day are ``input\01.txt`` or ``input\01_sample.txt``
        let mut filename: String = format!("input\\{:0>2}", day.to_string());
        if TEST_MODE {
            filename.push_str("_sample");
        }
        filename.push_str(".txt");

        // Read a file into a vector of Strings
        let buf = BufReader::new(std::fs::File::open(filename).expect("File not found."));
        let data: Vec<String> = buf.lines().map(|l| l.expect("Parse line error.")).collect();

        // Create the solver for the day, load and prepare the &data
        let mut now = std::time::Instant::now();
        let mut s: Box<dyn Solve> = match day {
            1 => Box::new(solutions::day01::Problem::new(&data)),
            2 => Box::new(solutions::day02::Problem::new(&data)),
            3 => Box::new(solutions::day03::Problem::new(&data)),
            4 => Box::new(solutions::day04::Problem::new(&data)),
            5 => Box::new(solutions::day05::Problem::new(&data)),
            6 => Box::new(solutions::day06::Problem::new(&data)),
            7 => Box::new(solutions::day07::Problem::new(&data)),
            8 => Box::new(solutions::day08::Problem::new(&data)),
            9 => Box::new(solutions::day09::Problem::new(&data)),
            10 => Box::new(solutions::day10::Problem::new(&data)),
            11 => Box::new(solutions::day11::Problem::new(&data)),
            12 => Box::new(solutions::day12::Problem::new(&data)),
            13 => Box::new(solutions::day13::Problem::new(&data)),
            14 => Box::new(solutions::day14::Problem::new(&data)),
            15 => Box::new(solutions::day15::Problem::new(&data)),
            16 => Box::new(solutions::day16::Problem::new(&data)),
            17 => Box::new(solutions::day17::Problem::new(&data)),
            18 => Box::new(solutions::day18::Problem::new(&data)),
            19 => Box::new(solutions::day19::Problem::new(&data)),
            20 => Box::new(solutions::day20::Problem::new(&data)),
            21 => Box::new(solutions::day21::Problem::new(&data)),
            22 => Box::new(solutions::day22::Problem::new(&data)),
            23 => Box::new(solutions::day23::Problem::new(&data)),
            24 => Box::new(solutions::day24::Problem::new(&data)),
            _ => Box::new(solutions::day25::Problem::new(&data)),
        };
        println!("{:0>2}:                 in {:>10?}", day, now.elapsed());

        // Solve each part for the day
        now = std::time::Instant::now();
        println!(" 1: {:15} in {:>10?}", s.p1(), now.elapsed());
        now = std::time::Instant::now();
        println!(" 2: {:15} in {:>10?}", s.p2(), now.elapsed());
    }
    println!("Total elapsed time:    {:>10?}", start.elapsed());
}
