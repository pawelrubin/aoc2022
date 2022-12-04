mod shared;

fn get_edges(line: String) -> [i32; 4] {
    let (a, b) = line.split_once(",").unwrap();
    let (a1, a2) = a.split_once("-").unwrap();
    let (b1, b2) = b.split_once("-").unwrap();

    [a1, a2, b1, b2].map(|e| e.parse().unwrap())
}

fn main() {
    let part1 = shared::get_lines()
        .map(get_edges)
        .filter(|[a1, a2, b1, b2]| (a1 >= b1 && a2 <= b2) || (b1 >= a1 && b2 <= a2))
        .count();

    let part2 = shared::get_lines()
        .map(get_edges)
        .filter(|[a1, a2, b1, b2]| (a2 >= b1 && a1 <= b1) || (b2 >= a1 && b1 <= a1))
        .count();

    println!("{}", part1);
    println!("{}", part2);
}
