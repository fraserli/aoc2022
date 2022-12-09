use criterion::{criterion_group, criterion_main, Criterion};

use aoc2022::solutions::*;

macro_rules! benchmark {
    ($module:ident) => {
        fn $module(c: &mut Criterion) {
            const INPUT: &str = include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/inputs/",
                stringify!($module),
                ".txt"
            ));
            c.bench_function(stringify!($module), |b| {
                b.iter(|| {
                    let data = $module::process_input(&INPUT);
                    $module::part1(&data);
                    $module::part2(&data);
                })
            });
        }
    };
}

macro_rules! benchmark_all {
    ($($module:ident), *) => {
        $(benchmark!($module);)*

        fn all(c: &mut Criterion) {
            c.bench_function("all", |b| {
                b.iter(|| {
                    $({
                        const INPUT: &str = include_str!(concat!(
                            env!("CARGO_MANIFEST_DIR"),
                            "/inputs/",
                            stringify!($module),
                            ".txt"
                        ));
                        let data = $module::process_input(&INPUT);
                        $module::part1(&data);
                        $module::part2(&data);
                    })*
                })
            });
        }

        criterion_group!(benches, $($module),*, all);
    };
}

benchmark_all!(day01, day02, day03, day04, day05, day06, day07, day08, day09);

criterion_main!(benches);
