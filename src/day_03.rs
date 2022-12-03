#![feature(iter_array_chunks)]

use std::{collections::HashSet, hash::Hash};
mod shared;

fn get_priority(item: char) -> i32 {
    match item as u32 {
        65..=90 => (item as u32 - 38) as i32,
        97..=122 => (item as u32 - 96) as i32,
        _ => panic!(),
    }
}

fn intersection<T>(sets: Vec<HashSet<T>>) -> HashSet<T>
where
    HashSet<T>: Clone,
    T: Eq,
    T: Hash,
{
    let mut sets = sets.clone();
    let (result, others) = sets.split_at_mut(1);
    let result = &mut result[0];
    for other in others {
        result.retain(|e| other.contains(e));
    }
    result.clone()
}

fn part1() -> i32 {
    fn get_common(rucksack: String) -> char {
        let (compartment1, compartment2) = rucksack.split_at(rucksack.len() / 2);
        *compartment1
            .chars()
            .collect::<HashSet<char>>()
            .intersection(&compartment2.chars().collect::<HashSet<char>>())
            .next()
            .unwrap()
    }

    shared::get_lines().map(get_common).map(get_priority).sum()
}

fn part2() -> i32 {
    fn get_common(group: [String; 3]) -> char {
        *intersection(group.map(|e| e.chars().collect::<HashSet<char>>()).to_vec())
            .iter()
            .next()
            .unwrap()
    }

    shared::get_lines()
        .array_chunks::<3>()
        .map(get_common)
        .map(get_priority)
        .sum()
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}
