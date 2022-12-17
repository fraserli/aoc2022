use std::collections::{HashSet, VecDeque};

use Cell::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Cell {
    O, /* Empty */
    X, /* Falling */
    S, /* Stopped */
}

type Row = [Cell; 7];
type Map = VecDeque<Row>;

struct State {
    map: Map,
    i: usize,
    j: usize,
    height: usize,
}

const EMPTY_ROW: Row = [O; 7];

#[rustfmt::skip]
const ROCKS: &[&[Row]] = &[
    &[
        [O, O, X, X, X, X, O],
    ],
    &[
        [O, O, O, X, O, O, O],
        [O, O, X, X, X, O, O],
        [O, O, O, X, O, O, O],
    ],
    &[
        [O, O, X, X, X, O, O],
        [O, O, O, O, X, O, O],
        [O, O, O, O, X, O, O],
    ],
    &[
        [O, O, X, O, O, O, O],
        [O, O, X, O, O, O, O],
        [O, O, X, O, O, O, O],
        [O, O, X, O, O, O, O],
    ],
    &[
        [O, O, X, X, O, O, O],
        [O, O, X, X, O, O, O],
    ],
];

pub fn process_input(input: &str) -> Vec<char> {
    input.trim().chars().collect()
}

pub fn part1(jet: &[char]) -> usize {
    let mut map: Map = Map::new();
    let mut j = 0;
    let mut height = 0;

    for i in 0..2022 {
        let rock = ROCKS[i % ROCKS.len()];
        map.extend([EMPTY_ROW; 3]);
        map.extend(rock);

        simulate_movement(&mut map, jet, &mut j);

        height += trim_map(&mut map);

        map.retain(|row| row.iter().any(|&s| s == X || s == S));
    }

    height + map.len()
}

pub fn part2(jet: &[char]) -> usize {
    const ITERATIONS: usize = 1000000000000;

    let mut map: Map = Map::new();
    let mut j = 0;
    let mut height = 0;

    let mut states: Vec<State> = Vec::new();
    let mut found_cycle = false;

    let mut i = 0;
    while i < ITERATIONS {
        let rock = ROCKS[i % ROCKS.len()];
        map.extend([EMPTY_ROW; 3]);
        map.extend(rock);

        simulate_movement(&mut map, jet, &mut j);

        map.retain(|row| row.iter().any(|&s| s == X || s == S));

        if i % 10 == 0 {
            let trimmed = trim_map(&mut map);
            height += trimmed;

            if !found_cycle && trimmed != 0 {
                for s in states.iter() {
                    if s.map == map
                        && i % ROCKS.len() == s.i % ROCKS.len()
                        && j % jet.len() == s.j % jet.len()
                    {
                        let cycle_len = i - s.i;
                        let cycle_height = height - s.height;
                        let iters = (ITERATIONS - i) / cycle_len;

                        height += iters * cycle_height;
                        i += iters * cycle_len;
                        found_cycle = true;
                    }
                }

                if found_cycle {
                    states.clear();
                }

                states.push(State {
                    map: map.clone(),
                    i,
                    j,
                    height,
                });
            }
        }

        i += 1;
    }

    height + map.len()
}

#[allow(dead_code)]
fn print_map(map: &Map) {
    for row in map.iter().rev() {
        print!("|");
        for cell in row {
            print!(
                "{}",
                match cell {
                    O => ".",
                    X => "@",
                    S => "#",
                }
            );
        }
        println!("|");
    }
    println!("+-------+\n");
}

fn trim_map(map: &mut Map) -> usize {
    map.push_back(EMPTY_ROW);

    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    queue.push_back((map.len() - 1, 0));
    visited.insert((map.len() - 1, 0));

    let mut lowest = map.len() - 1;

    while let Some((row, x)) = queue.pop_front() {
        if row < lowest {
            lowest = row;
        }

        let mut neighbours = Vec::new();
        if row > 0 {
            neighbours.push((row - 1, x));
        }
        if x > 1 {
            neighbours.push((row, x - 1));
        }
        if x < 6 {
            neighbours.push((row, x + 1));
        }

        for (a, b) in neighbours {
            if !visited.contains(&(a, b)) && map[a][b] != S {
                queue.push_back((a, b));
                visited.insert((a, b));
            }
        }
    }

    if lowest < map.len() - 1 {
        map.drain(..lowest);
    }

    map.retain(|row| row.iter().any(|&s| s == X || s == S));

    lowest
}

fn simulate_movement(map: &mut Map, jet: &[char], j: &mut usize) {
    loop {
        let dir = jet[*j % jet.len()];
        *j += 1;

        let mut blocked = false;
        'outer: for row in map.iter() {
            if (dir == '<' && row[0] == X) || (dir == '>' && row[6] == X) {
                blocked = true;
                break;
            }

            let it: Box<dyn Iterator<Item = usize>> = match dir {
                '<' => Box::new(1..7),
                '>' => Box::new((0..6).rev()),
                _ => panic!(),
            };
            for x in it {
                if row[x] == X {
                    match dir {
                        '<' => {
                            if row[x - 1] == S {
                                blocked = true;
                                break 'outer;
                            }
                        }
                        '>' => {
                            if row[x + 1] == S {
                                blocked = true;
                                break 'outer;
                            }
                        }
                        _ => panic!(),
                    }
                }
            }
        }

        if !blocked {
            for row in map.iter_mut() {
                let it: Box<dyn Iterator<Item = usize>> = match dir {
                    '<' => Box::new(1..7),
                    '>' => Box::new((0..6).rev()),
                    _ => panic!(),
                };
                for x in it {
                    if row[x] == X {
                        match dir {
                            '<' => {
                                if row[x] == X {
                                    row[x] = O;
                                    row[x - 1] = X;
                                }
                            }
                            '>' => {
                                if row[x] == X {
                                    row[x] = O;
                                    row[x + 1] = X;
                                }
                            }
                            _ => panic!(),
                        }
                    }
                }
            }
        }

        // Vertical movement
        let mut blocked = false;
        'outer: for row in 0..map.len() {
            for x in 0..7 {
                if map[row][x] == X && (row == 0 || map[row - 1][x] == S) {
                    blocked = true;
                    break 'outer;
                }
            }
        }

        if blocked {
            break;
        } else {
            for row in 0..map.len() {
                for x in 0..7 {
                    if map[row][x] == X {
                        map[row][x] = O;
                        map[row - 1][x] = X;
                    }
                }
            }
        }
    }

    for row in map {
        for x in row {
            if *x == X {
                *x = S;
            }
        }
    }
}
