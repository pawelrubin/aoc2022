mod shared;
use std::{ops::Rem, str};

enum Instruction {
    AddX(i32),
    Noop,
}

fn parse_instruction(s: &str) -> Instruction {
    if s == "noop" {
        Instruction::Noop
    } else if let Some(("addx", value)) = s.split_once(" ") {
        Instruction::AddX(value.parse().unwrap())
    } else {
        panic!()
    }
}

fn get_x_registry_values(instructions: &[Instruction]) -> Vec<i32> {
    // TODO: returns one extra cycle?
    instructions
        .iter()
        .fold((vec![1], 1), |(mut xs, mut x), instruction| {
            match instruction {
                Instruction::Noop => xs.push(x),
                Instruction::AddX(value) => {
                    xs.push(x);
                    x += value;
                    xs.push(x);
                }
            };
            (xs, x)
        })
        .0
}

fn part1(instructions: &[Instruction]) -> i32 {
    get_x_registry_values(instructions)
        .iter()
        .enumerate()
        .fold(0, |mut acc, (cycle, x_reg)| {
            [20, 60, 100, 140, 180, 220]
                .contains(&(cycle + 1))
                .then(|| acc += *x_reg * (cycle as i32 + 1));
            acc
        })
}

fn part2(instructions: &[Instruction]) -> String {
    get_x_registry_values(instructions).iter().enumerate().fold(
        String::new(),
        |mut crt, (cycle, x_reg)| {
            let crt_row = (cycle as i32).rem(40);

            if crt_row == 0 && cycle != 0 {
                crt.push('\n')
            }

            if (x_reg - 1..=x_reg + 1).contains(&crt_row) {
                crt.push('X');
            } else {
                crt.push('.');
            }

            crt
        },
    )
}

fn main() {
    let instructions: Vec<Instruction> = shared::get_lines()
        .map(|line| parse_instruction(&line))
        .collect();

    println!("{}", part1(&instructions));
    println!("{}", part2(&instructions));
}
