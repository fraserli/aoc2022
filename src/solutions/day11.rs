use std::collections::VecDeque;

#[derive(Clone, Copy, Debug)]
pub enum Op {
    Add,
    Multiply,
}

#[derive(Clone, Debug)]
pub struct Monkey {
    items: VecDeque<usize>,
    op: Op,
    op_val: Option<usize>,
    divisor: usize,
    if_true: usize,
    if_false: usize,
}

pub fn process_input(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|group| {
            let mut lines = group.lines();
            lines.next().unwrap();

            let items = lines.next().unwrap()[18..]
                .split(',')
                .map(|x| x.trim().parse().unwrap())
                .collect();

            let op_line = lines.next().unwrap();
            let op = match op_line[23..24].as_ref() {
                "+" => Op::Add,
                "*" => Op::Multiply,
                _ => panic!(),
            };
            let op_val = op_line[25..].parse().ok();

            let divisor = lines.next().unwrap()[21..].parse().unwrap();
            let if_true = lines.next().unwrap()[29..].parse().unwrap();
            let if_false = lines.next().unwrap()[30..].parse().unwrap();

            Monkey {
                items,
                op,
                op_val,
                divisor,
                if_true,
                if_false,
            }
        })
        .collect()
}

pub fn part1(data: &[Monkey]) -> usize {
    let mut monkeys = data.to_owned();

    let mut inspections = vec![0; monkeys.len()];

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while let Some(mut item) = monkeys[i].items.pop_front() {
                item = match monkeys[i].op {
                    Op::Add => item + monkeys[i].op_val.unwrap_or(item),
                    Op::Multiply => item * monkeys[i].op_val.unwrap_or(item),
                };
                item /= 3;
                let next_monkey = if item % monkeys[i].divisor == 0 {
                    monkeys[i].if_true
                } else {
                    monkeys[i].if_false
                };
                monkeys[next_monkey as usize].items.push_back(item);

                inspections[i] += 1;
            }
        }
    }

    inspections.sort();

    inspections[monkeys.len() - 1] * inspections[monkeys.len() - 2]
}

pub fn part2(data: &[Monkey]) -> usize {
    let mut monkeys = data.to_owned();
    let prod: usize = monkeys.iter().map(|m|m.divisor).product();

    let mut inspections = vec![0; monkeys.len()];

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            while let Some(mut item) = monkeys[i].items.pop_front() {
                item = match monkeys[i].op {
                    Op::Add => item + monkeys[i].op_val.unwrap_or(item),
                    Op::Multiply => item * monkeys[i].op_val.unwrap_or(item),
                };
                item %= prod;
                let next_monkey = if item % monkeys[i].divisor == 0 {
                    monkeys[i].if_true
                } else {
                    monkeys[i].if_false
                };
                monkeys[next_monkey as usize].items.push_back(item);

                inspections[i] += 1;
            }
        }
    }

    inspections.sort();

    inspections[monkeys.len() - 1] * inspections[monkeys.len() - 2]
}
