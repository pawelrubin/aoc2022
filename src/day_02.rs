use std::collections::HashMap;

mod shared;

fn play(results: HashMap<&str, i32>) -> i32 {
    shared::get_lines()
        .map(|e| results.get(e.as_str()).unwrap())
        .sum()
}

fn main() {
    let part1 = HashMap::from([
        ("A X", 4), // rock     vs rock     -> 3 (draw) + 1 (rock)     = 4
        ("A Y", 8), // rock     vs paper    -> 6 (win)  + 2 (paper)    = 8
        ("A Z", 3), // rock     vs scissors -> 0 (lost) + 3 (scissors) = 3
        ("B X", 1), // paper    vs rock     -> 0 (lost) + 1 (rock)     = 1
        ("B Y", 5), // paper    vs paper    -> 3 (draw) + 2 (paper)    = 5
        ("B Z", 9), // paper    vs scissors -> 6 (win)  + 3 (scissors) = 9
        ("C X", 7), // scissors vs rock     -> 6 (win)  + 1 (rock)     = 7
        ("C Y", 2), // scissors vs paper    -> 0 (lost) + 2 (paper)    = 2
        ("C Z", 6), // scissors vs scissors -> 3 (draw) + 3 (scissors) = 6
    ]);

    let part2 = HashMap::from([
        ("A X", 3), // rock     vs scissors -> 0 (lost) + 3 (scissors) = 3
        ("A Y", 4), // rock     vs rock     -> 3 (draw) + 1 (rock)     = 4
        ("A Z", 8), // rock     vs paper    -> 6 (win)  + 2 (paper)    = 8
        ("B X", 1), // paper    vs rock     -> 0 (lost) + 1 (rock)     = 1
        ("B Y", 5), // paper    vs paper    -> 3 (draw) + 2 (paper)    = 5
        ("B Z", 9), // paper    vs scissors -> 6 (win)  + 3 (scissors) = 9
        ("C X", 2), // scissors vs paper    -> 0 (lost) + 2 (paper)    = 2
        ("C Y", 6), // scissors vs scissors -> 3 (draw) + 3 (scissors) = 6
        ("C Z", 7), // scissors vs rock     -> 6 (win)  + 1 (scissors) = 7
    ]);

    println!("{}", play(part1));
    println!("{}", play(part2));
}
