mod shared;
mod takeuntil;

use takeuntil::TakeUntilExtension;

type Grid = Vec<Vec<u32>>;

fn read_grid(lines: impl Iterator<Item = String>) -> Grid {
    lines
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .fold(Vec::new(), |mut acc, row: Vec<u32>| {
            acc.push(row);
            acc
        })
}

fn get_tree_lines(grid: &Grid) -> impl Iterator<Item = (&u32, [Vec<&u32>; 4])> {
    grid.iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, tree)| {
                    (
                        tree,
                        [
                            (&row[..j]).iter().rev().collect(), // left
                            (&row[j + 1..]).iter().collect(),   // right
                            (&grid[..i]).into_iter().map(|row| &row[j]).rev().collect(), // up
                            (&grid[i + 1..]).into_iter().map(|row| &row[j]).collect(), // down
                        ],
                    )
                })
                .collect::<Vec<(&u32, [Vec<&u32>; 4])>>()
        })
        .flatten()
}

fn main() {
    let grid = read_grid(shared::get_lines());
    let tree_lines: Vec<_> = get_tree_lines(&grid).collect();

    let part1 = tree_lines
        .iter()
        .filter(|(tree, tree_lines)| {
            tree_lines
                .iter()
                .any(|line| line.iter().all(|other| *other < tree))
        })
        .count();

    println!("{}", part1);

    let part2 = tree_lines
        .iter()
        .map(|(tree, tree_lines)| {
            tree_lines
                .iter()
                .map(|line| line.iter().take_until(|other| **other >= tree).count())
                .fold(1, |acc, e| (acc * e as i32))
        })
        .max()
        .unwrap();

    println!("{}", part2);
}
