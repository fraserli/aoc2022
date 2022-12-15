use std::cmp::{max, min};
use std::collections::{HashSet, VecDeque};
use std::ops::RangeInclusive;

pub type Point = (i64, i64);
pub type Grid = HashSet<Point>;

pub fn process_input(input: &str) -> (Grid, i64) {
    let mut grid = Grid::new();
    let mut lowest = 0;

    for line in input.lines() {
        let points: Vec<Point> = line
            .split(" -> ")
            .map(|s| {
                let mut it = s.split(',');
                (
                    it.next().unwrap().parse().unwrap(),
                    it.next().unwrap().parse().unwrap(),
                )
            })
            .collect();

        for p in points.windows(2) {
            let a = p[0];
            let b = p[1];
            lowest = max(lowest, max(a.1, b.1));

            let it: Box<dyn Iterator<Item = Point>> = if a.0 != b.0 {
                Box::new((min(a.0, b.0)..=max(a.0, b.0)).zip(std::iter::repeat(a.1)))
            } else {
                Box::new(std::iter::repeat(a.0).zip(min(a.1, b.1)..=max(a.1, b.1)))
            };

            grid.extend(it);
        }
    }

    (grid, lowest)
}

pub fn part1((grid, lowest): &(Grid, i64)) -> usize {
    let mut sand = Grid::new();

    'sand: for _ in 0.. {
        let mut pos = (500, 0);

        'pos: loop {
            if pos.1 > *lowest {
                break 'sand;
            }

            for (x, y) in [(0, 1), (-1, 1), (1, 1)] {
                let new_pos = (pos.0 + x, pos.1 + y);
                if !grid.contains(&new_pos) && !sand.contains(&new_pos) {
                    pos = new_pos;
                    continue 'pos;
                }
            }

            break;
        }

        sand.insert(pos);
    }

    //show_grid(grid, &sand, 493..=503, 0..=10);
    //show_grid(grid, &sand, 445..=520, 0..=150);

    sand.len()
}

pub fn part2((grid, lowest): &(Grid, i64)) -> usize {
    let mut sand = Grid::new();
    let mut queue = VecDeque::new();

    queue.push_back((500, 0));
    sand.insert((500, 0));

    while let Some((x, y)) = queue.pop_front() {
        if y < *lowest + 1 {
            for p in [(x, y + 1), (x - 1, y + 1), (x + 1, y + 1)] {
                if !sand.contains(&p) && !grid.contains(&p) {
                    queue.push_back(p);
                    sand.insert(p);
                }
            }
        }
    }

    sand.len()
}

pub fn show_grid(
    rock: &Grid,
    sand: &Grid,
    x_range: RangeInclusive<i64>,
    y_range: RangeInclusive<i64>,
) {
    for y in y_range {
        for x in x_range.clone() {
            let point = (x, y);
            if point == (500, 0) {
                print!("+");
            } else if rock.contains(&point) {
                print!("#");
            } else if sand.contains(&point) {
                print!("o");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
