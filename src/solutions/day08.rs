pub struct Grid {
    trees: Vec<u8>,
    width: usize,
    height: usize,
}

impl Grid {
    fn get(&self, x: usize, y: usize) -> u8 {
        self.trees[x + y * self.width]
    }

    fn directions(&self, x: usize, y: usize) -> Vec<Box<dyn Iterator<Item = (usize, usize)>>> {
        let left = (0..x).rev().zip(std::iter::repeat(y));
        let right = (x + 1..self.width).zip(std::iter::repeat(y));
        let up = std::iter::repeat(x).zip((0..y).rev());
        let down = std::iter::repeat(x).zip(y + 1..self.height);

        vec![
            Box::new(left),
            Box::new(right),
            Box::new(up),
            Box::new(down),
        ]
    }

    fn visible(&self, x: usize, y: usize) -> bool {
        let height = self.get(x, y);

        'outer: for direction in self.directions(x, y) {
            for (a, b) in direction {
                if self.get(a, b) >= height {
                    continue 'outer;
                }
            }

            return true;
        }

        false
    }

    fn scenic_score(&self, x: usize, y: usize) -> usize {
        let height = self.get(x, y);
        let mut score = 1;

        for direction in self.directions(x, y) {
            let mut dirscore = 0;

            for (a, b) in direction {
                dirscore += 1;
                if self.get(a, b) >= height {
                    break;
                }
            }

            score *= dirscore;
        }

        score
    }
}

pub fn process_input(input: &str) -> Grid {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let mut trees = Vec::new();

    for line in input.lines() {
        for c in line.chars() {
            trees.push(c as u8 - 48);
        }
    }

    Grid {
        trees,
        width,
        height,
    }
}

pub fn part1(grid: &Grid) -> usize {
    let mut total = 0;

    for x in 0..grid.width {
        for y in 0..grid.height {
            if grid.visible(x, y) {
                total += 1;
            }
        }
    }

    total
}

pub fn part2(grid: &Grid) -> usize {
    let mut highest = 0;

    for x in 0..grid.width {
        for y in 0..grid.height {
            let score = grid.scenic_score(x, y);
            if score > highest {
                highest = score;
            }
        }
    }

    highest
}
