pub fn process_input(input: &str) -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
    let mut lines = input.lines();

    let count = (input.lines().next().unwrap().len() + 1) / 4;
    let mut stacks = vec![Vec::new(); count];

    while let Some(line) = lines.next() {
        if &line.trim()[..1] != "[" {
            break;
        }

        for i in 0..count {
            let c = line[i * 4 + 1..i * 4 + 2].chars().next().unwrap();
            if c == ' ' {
                continue;
            }

            stacks[i].push(c);
        }
    }

    for stack in stacks.iter_mut() {
        stack.reverse();
    }

    lines.next();

    let moves = lines
        .map(|l| {
            let mut it = l.split(' ');
            it.next();
            let first = it.next().unwrap().parse().unwrap();
            it.next();
            let second = it.next().unwrap().parse().unwrap();
            it.next();
            let third = it.next().unwrap().parse().unwrap();
            (first, second, third)
        })
        .collect();

    (stacks, moves)
}

pub fn part1(data: &(Vec<Vec<char>>, Vec<(usize, usize, usize)>)) -> String {
    let mut stacks = data.0.to_owned();

    for (count, src, dest) in data.1.iter() {
        for _ in 0..*count {
            let c = stacks[*src - 1].pop().unwrap();
            stacks[*dest - 1].push(c);
        }
    }

    output(&stacks)
}

pub fn part2(data: &(Vec<Vec<char>>, Vec<(usize, usize, usize)>)) -> String {
    let mut stacks = data.0.to_owned();

    for (count, src, dest) in data.1.iter() {
        let range = stacks[*src - 1].len() - *count..;
        let tmp: Vec<char> = stacks[*src - 1].drain(range).collect();
        stacks[*dest - 1].extend(tmp);
    }

    output(&stacks)
}

fn output(stacks: &[Vec<char>]) -> String {
    let mut out = String::with_capacity(stacks.len());

    for stack in stacks {
        out.push(*stack.last().unwrap());
    }

    out
}
