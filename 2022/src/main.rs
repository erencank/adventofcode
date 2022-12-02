use adventofcode::read_file;
use std::env;

mod solutions;

use solutions::day01::day01;
use solutions::day02::day02;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day: u8 = args[1].clone().parse().unwrap();
    let input = read_file("inputs", day);

    let (part_a, part_b) = match day {
        1 => day01(&input),
        2 => day02(&input),
        // 3 => day03(&input),
        // 4 => day04(&input),
        // 5 => day05(&input),
        // 6 => day06(&input),
        // 7 => day07(&input),
        // 8 => day08(&input),
        // 9 => day09(&input),
        // 10 => day10(&input),
        // 11 => day11(&input),
        // 12 => day12(&input),
        // 13 => day13(&input),
        // 14 => day14(&input),
        // 15 => day15(&input),
        // 16 => day16(&input),
        // 17 => day17(&input),
        //18 => day18(&input),
        //19 => day19(&input),
        //20 => day20(&input),
        // 21 => day21(&input),
        _ => ("".to_string(), "".to_string()),
    };
    println!("Part A: {}", part_a);
    println!("Part B: {}", part_b);
}
