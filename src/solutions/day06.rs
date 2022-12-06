use std::collections::HashSet;

pub fn process_input(input: &str) -> Vec<char> {
    input.chars().collect()
}

pub fn part1(data: &[char]) -> usize {
    for i in 0..data.len() - 4 {
        let buf = &data[i..i + 4];
        let set: HashSet<&char> = HashSet::from_iter(buf.iter());
        if set.len() == 4 {
            return i + 4;
        }
    }

    panic!();
}

pub fn part2(data: &[char]) -> usize {
    for i in 0..data.len() - 14 {
        let buf = &data[i..i + 14];
        let set: HashSet<&char> = HashSet::from_iter(buf.iter());
        if set.len() == 14 {
            return i + 14;
        }
    }

    panic!();
}
