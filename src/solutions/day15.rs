use std::cmp::{max, min};

pub struct Sensor {
    x: i64,
    y: i64,
    distance: i64,
}

impl Sensor {
    fn contains(&self, x: i64, y: i64) -> bool {
        (self.x - x).abs() + (self.y - y).abs() <= self.distance
    }
}

pub fn process_input(input: &str) -> Vec<Sensor> {
    input
        .lines()
        .map(|l| {
            let g: Vec<&str> = l.split(&[',', '=', ':']).collect();
            let x: i64 = g[1].parse().unwrap();
            let y: i64 = g[3].parse().unwrap();
            let a: i64 = g[5].parse().unwrap();
            let b: i64 = g[7].parse().unwrap();
            Sensor {
                x,
                y,
                distance: (x - a).abs() + (y - b).abs(),
            }
        })
        .collect()
}

pub fn part1(data: &[Sensor]) -> usize {
    let mut ranges = Vec::with_capacity(data.len());
    let mut lowest = std::i64::MAX;
    let mut highest = std::i64::MIN;

    for sensor in data {
        let width = sensor.distance - (sensor.y - 2000000).abs();
        if width >= 0 {
            lowest = min(lowest, sensor.x - width);
            highest = max(highest, sensor.x + width);
            ranges.push((sensor.x - width, sensor.x + width));
        }
    }

    (lowest..=highest)
        .filter(|x| ranges.iter().any(|(a, b)| a <= x && x < b))
        .count()
}

pub fn part2(data: &[Sensor]) -> i64 {
    for sensor in data {
        let radius = sensor.distance + 1;

        'outer: for a in 0..radius {
            let b = radius - a;
            let x = a + sensor.x;
            let y = b + sensor.y;

            if !(0..=4000000).contains(&x) || !(0..=4000000).contains(&y) {
                continue;
            }

            for s in data {
                if s.contains(x, y) {
                    continue 'outer;
                }
            }

            return x * 4000000 + y;
        }
    }

    panic!();
}
