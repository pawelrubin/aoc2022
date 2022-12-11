mod shared;

use auto_ops::*;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl_op_ex!(+ |a: &Point, b: &Point| -> Point { Point{x: a.x + b.x, y: a.y + b.y }});
impl_op_ex!(-|a: &Point, b: &Point| -> Point {
    Point {
        x: a.x - b.x,
        y: a.y - b.y,
    }
});
impl_op_ex!(+= |a: &mut Point, b: &Point| {a.x += b.x; a.y += b.y});

impl Point {
    fn origin() -> Point {
        Point { x: 0, y: 0 }
    }
    fn is_adjacent(&self, other: &Point) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }
    fn move_towards(&mut self, other: &Point) {
        self.x += other.x.cmp(&self.x) as i32;
        self.y += other.y.cmp(&self.y) as i32;
    }
}

fn parse_line(line: String) -> (Point, i32) {
    let (direction, steps) = line.split_once(" ").unwrap();
    let direction = match direction {
        "U" => Point { x: 0, y: 1 },
        "D" => Point { x: 0, y: -1 },
        "R" => Point { x: 1, y: 0 },
        "L" => Point { x: -1, y: 0 },
        _ => unreachable!(),
    };
    (direction, steps.parse().unwrap())
}

fn get_visited_by_tail(moves: &[(Point, i32)], inner_knots: usize) -> HashSet<Point> {
    let mut visited_by_tail: HashSet<Point> = HashSet::from([]);
    let mut head = Point::origin();
    let mut tail: Vec<Point> = (0..inner_knots + 1).map(|_| Point::origin()).collect();

    moves.iter().for_each(|(direction, steps)| {
        for _ in 1..=*steps {
            head += direction;

            let mut prev = head;

            for knot in tail.iter_mut() {
                if !knot.is_adjacent(&prev) {
                    knot.move_towards(&prev);
                }
                prev = *knot;
            }

            visited_by_tail.insert(*tail.last().unwrap());
        }
    });

    visited_by_tail
}

fn main() {
    let moves: Vec<(Point, i32)> = shared::get_lines().map(parse_line).collect();

    let part1 = get_visited_by_tail(&moves, 0).len();
    let part2 = get_visited_by_tail(&moves, 8).len();

    println!("{}", part1);
    println!("{}", part2);
}
