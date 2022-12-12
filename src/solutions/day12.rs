use std::collections::{BinaryHeap, HashMap};

pub struct Grid {
    cells: Vec<u8>,
    width: usize,
    height: usize,
    start: (usize, usize),
    end: (usize, usize),
}

impl Grid {
    fn get(&self, x: usize, y: usize) -> u8 {
        self.cells[x + y * self.width]
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct State {
    cell: (usize, usize),
    cost: usize,
    from: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn process_input(input: &str) -> Grid {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let cells: Vec<char> = input.chars().filter(|c| c.is_alphabetic()).collect();
    assert_eq!(cells.len(), width * height);

    let start_idx = cells.iter().position(|c| *c == 'S').unwrap();
    let end_idx = cells.iter().position(|c| *c == 'E').unwrap();
    let start = (start_idx % width, start_idx / width);
    let end = (end_idx % width, end_idx / width);

    let cells: Vec<u8> = cells
        .into_iter()
        .map(|c| match c {
            'S' => 'a' as u8,
            'E' => 'z' as u8,
            'a'..='z' => c as u8,
            _ => panic!(),
        })
        .collect();

    Grid {
        cells,
        width,
        height,
        start,
        end,
    }
}

pub fn part1(grid: &Grid) -> usize {
    pathfind(grid, false)
}

pub fn part2(grid: &Grid) -> usize {
    pathfind(grid, true)
}

/* Implemented with Dijkstra's algorithm */
fn pathfind(grid: &Grid, part2: bool) -> usize {
    let mut finished: HashMap<(usize, usize), State> = HashMap::with_capacity(grid.cells.len());
    let mut heap: BinaryHeap<State> = BinaryHeap::new();

    heap.push(State {
        cell: grid.start,
        cost: 0,
        from: grid.start,
    });

    while let Some(state) = heap.pop() {
        if state.cell == grid.end {
            return state.cost;
        } else if finished.contains_key(&state.cell) {
            continue;
        }

        let mut neighbours = Vec::with_capacity(4);
        if state.cell.0 != 0 {
            neighbours.push((state.cell.0 - 1, state.cell.1));
        }
        if state.cell.0 != grid.width - 1 {
            neighbours.push((state.cell.0 + 1, state.cell.1));
        }
        if state.cell.1 != 0 {
            neighbours.push((state.cell.0, state.cell.1 - 1));
        }
        if state.cell.1 != grid.height - 1 {
            neighbours.push((state.cell.0, state.cell.1 + 1));
        }

        let height = grid.get(state.cell.0, state.cell.1);
        for n in neighbours {
            let n_height = grid.get(n.0, n.1);
            if n_height <= height + 1 {
                heap.push(State {
                    cell: n,
                    cost: if part2 && grid.get(n.0, n.1) == 'a' as u8 {
                        0
                    } else {
                        state.cost + 1
                    },
                    from: state.cell,
                });
            }
        }

        if !finished.contains_key(&state.cell) {
            finished.insert(state.cell, state);
        }
    }

    panic!("No path");
}
