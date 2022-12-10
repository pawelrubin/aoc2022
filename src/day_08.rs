#![feature(exact_size_is_empty)]
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
            let left = &mut (&row[..j]).iter();
            let right = &mut (&row[j + 1..]).iter();
            let up = &mut (&grid[..i]).into_iter().map(|row| &row[j]);
            let down = &mut (&grid[i + 1..]).into_iter().map(|row| &row[j]);

            let is_visible = |other| other < tree;

            if left.all(is_visible)
                || right.all(is_visible)
                || up.all(is_visible)
                || down.all(is_visible)
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
            let left = (&row[..j]).iter().rev();
            let right = (&row[j + 1..]).iter();
            let up = (&grid[..i]).into_iter().map(|row| &row[j]).rev();
            let down = (&grid[i + 1..]).into_iter().map(|row| &row[j]);

            let left_viewing_distance = left.take_until(|other| *other >= tree).count();
            let right_viewing_distance = right.take_until(|other| *other >= tree).count();
            let up_viewing_distance = up.take_until(|other| *other >= tree).count();
            let down_viewing_distance = down.take_until(|other| *other >= tree).count();

            scenic_scores.push(
                left_viewing_distance
                    * right_viewing_distance
                    * up_viewing_distance
                    * down_viewing_distance,
            )
        }
    }
    scenic_scores.into_iter().max().unwrap()
}

fn main() {
    let grid = read_grid(shared::get_lines());
    println!("{:?}", count_visible_trees(&grid));
    println!("{:?}", find_best_scenic_score(&grid));
}
