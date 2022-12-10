use std::{self, collections::HashMap};

mod shared;

fn accumulate(iter: Vec<String>) -> Vec<String> {
    // TODO: don't clone data around
    let mut current = iter.first().unwrap().clone();
    let mut result = vec![current.clone()];
    for item in &iter[1..] {
        let next_current = current.to_owned() + item;
        result.push(next_current.clone());
        current = next_current;
    }
    result
}

fn main() {
    let mut dir_sizes = HashMap::<String, u32>::new();
    let mut cwd = vec![];

    for line in shared::get_lines() {
        match line.split(" ").collect::<Vec<&str>>()[..] {
            ["$", "cd", "/"] => cwd = vec!["/".to_owned()],
            ["$", "cd", ".."] => _ = cwd.pop(),
            ["$", "cd", path] => _ = cwd.push(path.to_owned() + "/"),
            ["$", "ls"] | ["dir", _] => (),
            [file_size, _] => {
                for path in accumulate(cwd.clone()) {
                    *dir_sizes.entry(path).or_insert(0) += file_size.parse::<u32>().unwrap();
                }
            }
            _ => unreachable!(),
        };
    }

    let part1: u32 = dir_sizes
        .clone()
        .into_values()
        .filter(|size| *size <= 100_000)
        .sum();

    println!("{}", part1);

    let total_size = dir_sizes.get("/").clone().unwrap();
    let unused_space = 70000000 - total_size;
    let required_unused_space: u32 = 30_000_000;
    let part2: u32 = dir_sizes
        .into_values()
        .map(|size| size + unused_space)
        .filter(|size| *size >= required_unused_space)
        .min()
        .unwrap()
        - unused_space;

    println!("{}", part2);
}
