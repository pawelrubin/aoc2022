use std::collections::HashMap;

mod common;

fn play(results: HashMap<String, i32>) -> i32 {
    common::get_lines().map(|e| results.get(&e).unwrap()).sum()
}

fn main() {
    let part1 = HashMap::from([
        ("A X".to_string(), 4), // rock     vs rock     -> 3 (draw) + 1 (rock)     = 4
        ("A Y".to_string(), 8), // rock     vs paper    -> 6 (win)  + 2 (paper)    = 8
        ("A Z".to_string(), 3), // rock     vs scissors -> 0 (lost) + 3 (scissors) = 3
        ("B X".to_string(), 1), // paper    vs rock     -> 0 (lost) + 1 (rock)     = 1
        ("B Y".to_string(), 5), // paper    vs paper    -> 3 (draw) + 2 (paper)    = 5
        ("B Z".to_string(), 9), // paper    vs scissors -> 6 (win)  + 3 (scissors) = 9
        ("C X".to_string(), 7), // scissors vs rock     -> 6 (win)  + 1 (rock)     = 7
        ("C Y".to_string(), 2), // scissors vs paper    -> 0 (lost) + 2 (paper)    = 2
        ("C Z".to_string(), 6), // scissors vs scissors -> 3 (draw) + 3 (scissors) = 6
    ]);

    let part2 = HashMap::from([
        ("A X".to_string(), 3), // rock     vs scissors -> 0 (lost) + 3 (scissors) = 3
        ("A Y".to_string(), 4), // rock     vs rock     -> 3 (draw) + 1 (rock)     = 4
        ("A Z".to_string(), 8), // rock     vs paper    -> 6 (win)  + 2 (paper)    = 8
        ("B X".to_string(), 1), // paper    vs rock     -> 0 (lost) + 1 (rock)     = 1
        ("B Y".to_string(), 5), // paper    vs paper    -> 3 (draw) + 2 (paper)    = 5
        ("B Z".to_string(), 9), // paper    vs scissors -> 6 (win)  + 3 (scissors) = 9
        ("C X".to_string(), 2), // scissors vs paper    -> 0 (lost) + 2 (paper)    = 2
        ("C Y".to_string(), 6), // scissors vs scissors -> 3 (draw) + 3 (scissors) = 6
        ("C Z".to_string(), 7), // scissors vs rock     -> 6 (win)  + 1 (scissors) = 7
    ]);

    println!("{}", play(part1));
    println!("{}", play(part2));
}
