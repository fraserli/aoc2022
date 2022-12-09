use std::cmp::Ordering::*;
use std::collections::HashSet;

pub type Position = (i64, i64);

pub fn process_input(input: &str) -> Vec<(char, i64)> {
    input
        .lines()
        .map(|l| {
            let mut it = l.split(' ');
            (
                it.next().unwrap().chars().next().unwrap(),
                it.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

pub fn part1(data: &[(char, i64)]) -> usize {
    let mut positions: HashSet<Position> = HashSet::new();

    let mut head: Position = (0, 0);
    let mut tail: Position = (0, 0);

    positions.insert(tail);

    for (dir, mag) in data {
        for new_head in steps(head, *dir, *mag) {
            if !(new_head.0 - 1 <= tail.0
                && tail.0 <= new_head.0 + 1
                && new_head.1 - 1 <= tail.1
                && tail.1 <= new_head.1 + 1)
            {
                tail = head;
                positions.insert(tail);
            }

            head = new_head;
        }
    }

    positions.len()
}

pub fn part2(data: &[(char, i64)]) -> usize {
    let mut positions: HashSet<Position> = HashSet::new();

    let mut segments = [(0, 0); 10];

    positions.insert(segments[9]);

    for (dir, mag) in data {
        for new_head in steps(segments[0], *dir, *mag) {
            segments[0] = new_head;

            for i in 1..segments.len() {
                let segment = segments[i];
                let prev = segments[i - 1];

                if !(prev.0 - 1 <= segment.0
                    && segment.0 <= prev.0 + 1
                    && prev.1 - 1 <= segment.1
                    && segment.1 <= prev.1 + 1)
                {
                    let x = match prev.0.cmp(&segment.0) {
                        Less => segment.0 - 1,
                        Greater => segment.0 + 1,
                        Equal => segment.0,
                    };

                    let y = match prev.1.cmp(&segment.1) {
                        Less => segment.1 - 1,
                        Greater => segment.1 + 1,
                        Equal => segment.1,
                    };

                    segments[i] = (x, y);
                }
            }

            positions.insert(segments[9]);
        }
    }

    positions.len()
}

fn steps(head: Position, dir: char, mag: i64) -> Box<dyn Iterator<Item = Position>> {
    match dir {
        'R' => Box::new((head.0 + 1..head.0 + mag + 1).zip(std::iter::repeat(head.1))),
        'L' => Box::new((head.0 - mag..head.0).rev().zip(std::iter::repeat(head.1))),
        'D' => Box::new(std::iter::repeat(head.0).zip((head.1 - mag..head.1).rev())),
        'U' => Box::new(std::iter::repeat(head.0).zip(head.1 + 1..head.1 + mag + 1)),
        _ => panic!(),
    }
}
