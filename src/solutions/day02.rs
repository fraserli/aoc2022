pub fn process_input(input: &str) -> Vec<(&str, &str)> {
    input
        .lines()
        .map(|l| {
            let mut it = l.split(' ');
            (it.next().unwrap(), it.next().unwrap())
        })
        .collect()
}

pub fn part1(data: &[(&str, &str)]) -> i64 {
    data.iter().map(|(a, b)| score(a, b)).sum()
}

pub fn part2(data: &[(&str, &str)]) -> i64 {
    data.iter().map(|(a, b)| score2(a, b)).sum()
}

fn score(a: &str, b: &str) -> i64 {
    let score = match b {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => panic!(),
    };

    if (a == "A" && b == "X") || (a == "B" && b == "Y") || (a == "C" && b == "Z") {
        score + 3
    } else if (a == "A" && b == "Y") || (a == "B" && b == "Z") || (a == "C" && b == "X") {
        score + 6
    } else {
        score
    }
}

fn score2(a: &str, b: &str) -> i64 {
    let score = match b {
        "X" => 0,
        "Y" => 3,
        "Z" => 6,
        _ => panic!(),
    };

    if (b == "X" && a == "A") || (b == "Y" && a == "C") || (b == "Z" && a == "B") {
        // scissors
        score + 3
    } else if (b == "X" && a == "B") || (b == "Y" && a == "A") || (b == "Z" && a == "C") {
        // rock
        score + 1
    } else {
        // paper
        score + 2
    }
}
