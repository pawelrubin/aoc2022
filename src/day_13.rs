#![feature(array_chunks)]
mod shared;

use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(untagged)]
enum Data {
    Single(i32),
    List(Packet),
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Ord)]
struct Packet(Vec<Data>);

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        for (left, right) in (&self.0).into_iter().zip(&other.0) {
            match (left, right) {
                (Data::Single(x), Data::Single(y)) => match x.partial_cmp(&y) {
                    Some(Ordering::Equal) => continue,
                    result => return result,
                },
                (Data::List(xs), Data::List(ys)) => match xs.partial_cmp(&ys) {
                    Some(Ordering::Equal) => continue,
                    result => return result,
                },
                (Data::Single(x), Data::List(ys)) => {
                    return Packet(vec![Data::Single(*x)]).partial_cmp(&ys)
                }
                (Data::List(xs), Data::Single(y)) => {
                    return xs.partial_cmp(&Packet(vec![Data::Single(*y)]))
                }
            }
        }

        self.0.len().partial_cmp(&other.0.len())
    }
}

fn main() {
    let packets: Vec<Packet> = shared::get_lines()
        .filter(|line| !line.is_empty())
        .map(|line| serde_json::from_str::<Packet>(&line).unwrap())
        .collect();

    let part1: i32 = packets
        .array_chunks::<2>()
        .enumerate()
        .filter(|(_, [a, b])| a <= b)
        .map(|(i, _)| i as i32 + 1)
        .sum();

    println!("{}", part1);

    let dividers: Vec<Packet> = vec![
        serde_json::from_str("[[2]]").unwrap(),
        serde_json::from_str("[[6]]").unwrap(),
    ];

    let mut packets = [dividers.clone(), packets].concat();

    packets.sort();

    let part2 = packets
        .iter()
        .enumerate()
        .filter(|(_, e)| dividers.contains(e))
        .map(|(i, _)| i + 1)
        .fold(1, |a, b| a * b);

    println!("{}", part2);
}
