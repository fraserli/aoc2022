use std::time::Instant;

use aoc2022::solutions::*;

macro_rules! run {
    ($module:ident) => {{
        const INPUT: &str = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/inputs/",
            stringify!($module),
            ".txt"
        ));
        let data = $module::process_input(&INPUT);
        println!("\x1b[1;34mPart 1\x1b[0m\n{}", $module::part1(&data));
        println!("\x1b[1;34mPart 2\x1b[0m\n{}", $module::part2(&data));
    }};
}

fn main() {
    let day: usize = std::env::args().nth(1).unwrap().parse().unwrap();

    let t = Instant::now();

    match day {
        1 => run!(day01),
        2 => run!(day02),
        3 => run!(day03),
        4 => run!(day04),
        5 => run!(day05),
        6 => run!(day06),
        7 => run!(day07),
        8 => run!(day08),
        9 => run!(day09),
        10 => run!(day10),
        11 => run!(day11),
        12 => run!(day12),
        13 => run!(day13),
        14 => run!(day14),
        15 => run!(day15),
        16 => run!(day16),
        17 => run!(day17),
        18 => run!(day18),
        20 => run!(day20),
        _ => panic!("Invalid day: {}", day),
    };

    println!(
        "\nFinished in {:.4} ms",
        (Instant::now() - t).as_secs_f64() * 1000.0
    );
}
