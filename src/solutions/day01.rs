pub fn process_input(input: &str) -> Vec<i64> {
    let mut elves: Vec<i64> = input
        .split("\n\n")
        .map(|e| e.lines().map(|l| l.parse::<i64>().unwrap()).sum())
        .collect();
    elves.sort_unstable();
    elves
}

pub fn part1(data: &[i64]) -> i64 {
    *data.last().unwrap()
}

pub fn part2(data: &[i64]) -> i64 {
    data[data.len() - 3..].iter().sum()
}
