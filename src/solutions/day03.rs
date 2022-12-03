use std::collections::HashSet;

pub fn process_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part1(data: &[&str]) -> i64 {
    let mut total = 0;

    for line in data {
        let a: HashSet<char> = HashSet::from_iter(&mut line[..line.len() / 2].chars());
        let b: HashSet<char> = HashSet::from_iter(&mut line[line.len() / 2..].chars());
        total += a.intersection(&b).map(|c| priority(*c)).sum::<i64>()
    }

    total
}

pub fn part2(data: &[&str]) -> i64 {
    let mut total = 0;

    for i in 0..data.len() / 3 {
        let a: HashSet<char> = HashSet::from_iter(data[i * 3].chars());
        let b: HashSet<char> = HashSet::from_iter(data[i * 3 + 1].chars());
        let c: HashSet<char> = HashSet::from_iter(data[i * 3 + 2].chars());
        for x in a.iter() {
            if b.contains(x) && c.contains(x) {
                total += priority(*x);
            }
        }
    }

    total
}

fn priority(c: char) -> i64 {
    if c.is_lowercase() {
        c as i64 - 96
    } else {
        c as i64 - 38
    }
}
