use std::collections::{BinaryHeap, HashSet};

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn neighbours(&'_ self) -> impl Iterator<Item = Self> + '_ {
        [
            (1, 0, 0),
            (-1, 0, 0),
            (0, 1, 0),
            (0, -1, 0),
            (0, 0, 1),
            (0, 0, -1),
        ]
        .into_iter()
        .map(|(a, b, c)| Point {
            x: self.x + a,
            y: self.y + b,
            z: self.z + c,
        })
    }

    fn distance(&self, other: &Self) -> i64 {
        (((other.x - self.x) * (other.x - self.x)
            + (other.y - self.y) * (other.x - self.y)
            + (other.z - self.z) * (other.z - self.z)) as f64)
            .sqrt()
            .floor() as i64
    }
}

pub fn process_input(input: &str) -> HashSet<Point> {
    input
        .lines()
        .map(|l| {
            let mut it = l.split(',');
            Point {
                x: it.next().unwrap().parse().unwrap(),
                y: it.next().unwrap().parse().unwrap(),
                z: it.next().unwrap().parse().unwrap(),
            }
        })
        .collect()
}

pub fn part1(points: &HashSet<Point>) -> usize {
    let mut total = 0;

    for p in points.iter() {
        for n in p.neighbours() {
            if !points.contains(&n) {
                total += 1
            }
        }
    }

    total
}

pub fn part2(points: &HashSet<Point>) -> usize {
    let mut total = 0;

    let mut cache: HashSet<Point> = HashSet::new(); /* Known unenclosed points */

    for p in points.iter() {
        for n in p.neighbours() {
            if !points.contains(&n) && !enclosed(points, &mut cache, &n) {
                total += 1;
            }
        }
    }

    total
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct State {
    point: Point,
    g_cost: i64,
    h_cost: i64,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (other.g_cost + other.h_cost).cmp(&(self.g_cost + self.h_cost))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn enclosed(points: &HashSet<Point>, cache: &mut HashSet<Point>, start: &Point) -> bool {
    const END: Point = Point {
        x: 20,
        y: 20,
        z: 20,
    };

    let mut visited: HashSet<Point> = HashSet::new();
    let mut heap: BinaryHeap<State> = BinaryHeap::new();

    heap.push(State {
        point: *start,
        g_cost: 0,
        h_cost: start.distance(&END),
    });

    while let Some(state) = heap.pop() {
        if state.point == END || cache.contains(&state.point) {
            cache.extend(visited);
            return false;
        }

        for n in state.point.neighbours() {
            if !points.contains(&n) && !visited.contains(&n) {
                heap.push(State {
                    point: n,
                    g_cost: state.g_cost + 1,
                    h_cost: n.distance(&END),
                });
                visited.insert(n);
            }
        }
    }

    true
}
