pub fn process_input(input: &str) -> Vec<(usize, i64)> {
    input
        .lines()
        .enumerate()
        .map(|(i, l)| (i, l.parse().unwrap()))
        .collect()
}

pub fn part1(data: &[(usize, i64)]) -> i64 {
    let mut numbers = data.to_vec();
    mix(&mut numbers);
    coords(&numbers)
}

pub fn part2(data: &[(usize, i64)]) -> i64 {
    let mut numbers: Vec<(usize, i64)> =
        data.iter().map(|&(i, num)| (i, num * 811589153)).collect();

    for _ in 0..10 {
        mix(&mut numbers);
    }

    coords(&numbers)
}

fn mix(numbers: &mut Vec<(usize, i64)>) {
    for i in 0..numbers.len() {
        let idx = numbers.iter().position(|(j, _)| (i == *j)).unwrap();
        let num = numbers[idx].1;

        let len = numbers.len() as i64;
        let mut x = (((idx as i64 + num) % (len - 1)) + (len - 1)) % (len - 1);
        if num < 0 && x == 0 {
            x = len - 1;
        } else if num > 0 && x == len - 1 {
            x = 0;
        }

        let new_idx = x as usize;
        let a = numbers.remove(idx);
        numbers.insert(new_idx, a);
    }
}

fn coords(numbers: &Vec<(usize, i64)>) -> i64 {
    let zero = numbers.iter().position(|(_, num)| *num == 0).unwrap();
    numbers[(zero + 1000) % numbers.len()].1
        + numbers[(zero + 2000) % numbers.len()].1
        + numbers[(zero + 3000) % numbers.len()].1
}
