use take_until::TakeUntilExt;

mod shared;

type Grid = Vec<Vec<u32>>;

fn read_grid(lines: impl Iterator<Item = String>) -> Grid {
    lines
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .fold(Vec::new(), |mut acc, row: Vec<u32>| {
            acc.push(row);
            acc
        })
}

fn count_visible_trees(grid: &Grid) -> u32 {
    let mut count = 0;
    for (i, row) in (&grid).into_iter().enumerate() {
        for (j, tree) in row.into_iter().enumerate() {
            let tree_lines: [Vec<&u32>; 4] = [
                (&row[..j]).iter().collect(),
                (&row[j + 1..]).iter().collect(),
                (&grid[..i]).into_iter().map(|row| &row[j]).collect(),
                (&grid[i + 1..]).into_iter().map(|row| &row[j]).collect(),
            ];

            if tree_lines
                .iter()
                .any(|line| line.iter().all(|other| *other < tree))
            {
                count += 1;
            }
        }
    }
    count
}

fn find_best_scenic_score(grid: &Grid) -> usize {
    let mut scenic_scores = vec![];
    for (i, row) in (&grid).into_iter().enumerate() {
        for (j, tree) in row.into_iter().enumerate() {
            let tree_lines: [Vec<&u32>; 4] = [
                (&row[..j]).iter().rev().collect(),
                (&row[j + 1..]).iter().collect(),
                (&grid[..i]).into_iter().map(|row| &row[j]).rev().collect(),
                (&grid[i + 1..]).into_iter().map(|row| &row[j]).collect(),
            ];

            let scenic_score: usize = tree_lines
                .map(|line| line.iter().take_until(|other| **other >= tree).count())
                .iter()
                .fold(1, |acc, e| (acc * e));

            scenic_scores.push(scenic_score)
        }
    }
    scenic_scores.into_iter().max().unwrap()
}

fn main() {
    let grid = read_grid(shared::get_lines());
    println!("{:?}", count_visible_trees(&grid));
    println!("{:?}", find_best_scenic_score(&grid));
}
