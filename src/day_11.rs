use std::{
    collections::{HashSet, VecDeque},
    fmt::Debug,
};

use regex::Regex;

mod shared;

#[derive(Clone)]
struct Monkey {
    items: VecDeque<i64>,
    operation: fn(&i64, &i64) -> i64,
    value: i64,
    divisor: i64,
    if_true: usize,
    if_false: usize,
}

impl Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Monkey")
            .field("items", &self.items)
            .field("value", &self.value)
            .field("divisor", &self.divisor)
            .finish()
    }
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    let re = Regex::new(r"Monkey \d:\n  Starting items: ((?:\d+)(?:, (?:\d+))*)\n  Operation: new = old ([*+] (?:(?:\d+)|old))\n  Test: divisible by (\d+)\n    If true: throw to monkey (\d+)\n    If false: throw to monkey (\d+)").unwrap();

    re.captures_iter(input)
        .fold(Vec::new(), |mut monkeys, cap| {
            let (op, val) = (&cap[2]).split_once(" ").unwrap();
            let val = val.parse::<i64>().ok();

            let monkey = Monkey {
                items: (&cap[1])
                    .split(", ")
                    .collect::<Vec<_>>()
                    .iter()
                    .map(|e| e.parse().unwrap())
                    .collect(),

                operation: match (op, val) {
                    ("*", Some(_)) => |item: &i64, val: &i64| item * val,
                    ("+", Some(_)) => |item: &i64, val: &i64| item + val,
                    ("*", None) => |item: &i64, _: &i64| item * item,
                    ("+", None) => |item: &i64, _: &i64| item + item,
                    _ => unreachable!(),
                },
                value: val.unwrap_or_default(),
                divisor: (&cap[3]).parse::<i64>().unwrap(),
                if_true: (&cap[4]).parse().unwrap(),
                if_false: (&cap[5]).parse().unwrap(),
            };
            monkeys.push(monkey);
            monkeys
        })
}

fn main() {
    let input: String = shared::get_contents();
    let mut monkeys = parse_monkeys(&input);
    let mut stats: Vec<i64> = vec![0; monkeys.len()];
    let modulo: i64 = monkeys.iter().map(|m| m.divisor).fold(1, |a, b| a * b);

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];

            let mut stack: Vec<(usize, i64)> = Vec::new();
            while let Some(item) = monkey.items.pop_front() {
                let item = (monkey.operation)(&item, &monkey.value) % modulo;
                let next_monkey_index = if item % monkey.divisor == 0 {
                    monkey.if_true
                } else {
                    monkey.if_false
                };
                stack.push((next_monkey_index, item));
                stats[i] += 1;
            }

            while let Some((i, e)) = stack.pop() {
                monkeys[i].items.push_back(e);
            }
        }
    }
    stats.sort();
    let part1 = stats.iter().rev().take(2).fold(1, |a, b| a * b);
    println!("{}", part1);
}
