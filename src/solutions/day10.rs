use std::iter::Peekable;
use std::slice::Iter;

const CRT_WIDTH: i64 = 40;

pub enum Instruction {
    AddX(i64),
    NoOp,
}

struct State<'a> {
    instructions: Peekable<Iter<'a, Instruction>>,
    pub clock: i64,
    pub x: i64,
    progress: usize,
}

impl<'a> State<'a> {
    fn new(instructions: &'a [Instruction]) -> Self {
        Self {
            instructions: instructions.iter().peekable(),
            clock: 0,
            x: 0,
            progress: 0,
        }
    }

    fn finished(&mut self) -> bool {
        self.instructions.peek().is_none()
    }

    fn tick(&mut self) {
        match self.instructions.peek().unwrap() {
            Instruction::NoOp => {
                self.instructions.next().unwrap();
            }
            Instruction::AddX(num) => {
                if self.progress == 1 {
                    self.x += num;
                    self.instructions.next();
                    self.progress = 0;
                } else {
                    self.progress += 1;
                }
            }
        }

        self.clock += 1;
    }
}

pub fn process_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| {
            let mut it = l.split(' ');
            match it.next().unwrap() {
                "noop" => Instruction::NoOp,
                "addx" => {
                    let num = it.next().unwrap().parse().unwrap();
                    Instruction::AddX(num)
                }
                _ => panic!(),
            }
        })
        .collect()
}

pub fn part1(data: &[Instruction]) -> i64 {
    let mut state = State::new(data);
    let mut total = 0;

    while !state.finished() {
        state.tick();
        if [20, 60, 100, 140, 180, 220].contains(&(state.clock + 1)) {
            total += (state.clock + 1) * (state.x + 1);
        }
    }

    total

}

pub fn part2(data: &[Instruction]) -> String {
    let mut state = State::new(data);
    let mut output = String::new();

    while !state.finished() {
        let pixel = state.clock % CRT_WIDTH;
        output.push_str(if state.x <= pixel && pixel <= state.x + 2 {
            "\x1b[7m \x1b[0m"
        } else {
            " "
        });

        if pixel == CRT_WIDTH - 1 {
            output.push('\n')
        }

        state.tick();
    }

    output
}
