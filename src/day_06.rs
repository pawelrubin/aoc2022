use std::{collections::HashSet, hash::Hash};

mod shared;

fn has_unique_elements<T>(iter: &Vec<T>) -> bool
where
    T: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(|x| uniq.insert(x))
}

fn find_marker_position(marker_size: usize) -> usize {
    let line = shared::get_lines().next().unwrap();
    let mut binding = line.chars().enumerate();
    let input = binding.by_ref();
    let mut marker: Vec<char> = input.take(marker_size).map(|(_, c)| c).collect();

    if has_unique_elements(&marker) {
        return marker_size;
    }

    for (i, c) in input {
        marker.remove(0);
        marker.push(c);
        if has_unique_elements(&marker) {
            return i + 1;
        }
    }

    panic!();
}
fn main() {
    println!("{}", find_marker_position(4));  // part 1
    println!("{}", find_marker_position(14));  // part 2
}
