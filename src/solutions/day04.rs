pub fn process_input(input: &str) -> Vec<(i64, i64, i64, i64)> {
    input
        .lines()
        .map(|l| {
            let mut it = l.split(&['-', ',']);
            let a1 = it.next().unwrap().parse().unwrap();
            let a2 = it.next().unwrap().parse().unwrap();
            let b1 = it.next().unwrap().parse().unwrap();
            let b2 = it.next().unwrap().parse().unwrap();
            (a1, a2, b1, b2)
        })
        .collect()
}

pub fn part1(data: &[(i64, i64, i64, i64)]) -> usize {
    data.iter()
        .filter(|(a1, a2, b1, b2)| {
            (a1 <= b1 && b1 <= a2 && a1 <= b2 && b2 <= a2)
                || (b1 <= a1 && a1 <= b2 && b1 <= a2 && a2 <= b2)
        })
        .count()
}

pub fn part2(data: &[(i64, i64, i64, i64)]) -> usize {
    data.iter()
        .filter(|(a1, a2, b1, b2)| a1 <= b2 && b1 <= a2)
        .count()
}
