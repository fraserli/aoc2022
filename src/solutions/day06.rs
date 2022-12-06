use std::collections::HashSet;

pub fn process_input(input: &str) -> Vec<char> {
    input.chars().collect()
}

pub fn part1(data: &[char]) -> usize {
    let mut set: HashSet<&char> = HashSet::from_iter(data[0..4].iter());

    for i in 0..data.len() - 4 {
        if set.len() == 4 {
            return i + 4;
        }

        if !data[i + 1..i + 4].contains(&data[i]) {
            set.remove(&data[i]);
        }

        set.insert(&data[i + 4]);
    }

    panic!();
}

pub fn part2(data: &[char]) -> usize {
    let mut set: HashSet<&char> = HashSet::from_iter(data[0..14].iter());

    for i in 0..data.len() - 14 {
        if set.len() == 14 {
            return i + 14;
        }

        if !data[i + 1..i + 14].contains(&data[i]) {
            set.remove(&data[i]);
        }

        set.insert(&data[i + 14]);
    }

    panic!();
}
