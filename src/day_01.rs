mod common;

fn main() {
    // count calories for every elf
    let mut elves: Vec<_> = common::get_lines()
        .collect::<Vec<String>>()
        .split(String::is_empty)
        .map(|group| group.iter().map(|e| e.parse::<i32>().unwrap()).sum::<i32>())
        .collect();

    // reverse sort
    elves.sort_by(|a, b| b.cmp(a));

    // find top elves
    let top_elf: i32 = elves.iter().take(1).sum();
    let top_3_elves_sum: i32 = elves.iter().take(3).sum();

    println!("{}", top_elf);
    println!("{}", top_3_elves_sum);
}
