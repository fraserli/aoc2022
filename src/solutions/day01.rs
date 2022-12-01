pub fn process_input(input: &str) -> Vec<i64> {
    let mut elves = Vec::new();

    for elf in input.split("\n\n") {
        let mut total = 0;
        for line in elf.lines() {
            let num: i64 = line.parse().unwrap();
            total += num;
        }
        elves.push(total);
    }

    elves
}

pub fn part1(data: &[i64]) -> i64 {
    *data.iter().max().unwrap()
}

pub fn part2(data: &[i64]) -> i64 {
    let mut elves = data.to_vec();
    elves.sort();
    elves[elves.len() - 3..].iter().sum()
}
