mod shared;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
#[serde(untagged)]
enum Data {
    Single(i32),
    List(Vec<Data>),
}

type Packet = Vec<Data>;

enum OrderResult {
    Right,
    Wrong,
    LeftExhausted,
    Equal,
    RightExhausted,
}

impl OrderResult {
    fn from(c: bool) -> OrderResult {
        if c {
            OrderResult::Right
        } else {
            OrderResult::Wrong
        }
    }
}

fn are_ordered(a: &Packet, b: &Packet) -> bool {
    fn check_order(a: &Packet, b: &Packet) -> OrderResult {
        for (left, right) in a.iter().zip(b) {
            match (left, right) {
                (Data::Single(x), Data::Single(y)) => {
                    if x == y {
                        continue;
                    }

                    return OrderResult::from(x < y);
                }
                (Data::List(xs), Data::List(ys)) => match check_order(xs, ys) {
                    OrderResult::Right | OrderResult::LeftExhausted => return OrderResult::Right,
                    OrderResult::Wrong | OrderResult::RightExhausted => return OrderResult::Wrong,
                    OrderResult::Equal => continue,
                },
                (Data::Single(x), Data::List(ys)) => {
                    return check_order(&vec![Data::Single(*x)], ys)
                }
                (Data::List(xs), Data::Single(y)) => {
                    return check_order(xs, &vec![Data::Single(*y)])
                }
            }
        }

        if a.len() < b.len() {
            return OrderResult::LeftExhausted;
        }

        if a.len() == b.len() {
            return OrderResult::Equal;
        }

        return OrderResult::RightExhausted;
    }

    matches!(
        check_order(a, b),
        OrderResult::Right | OrderResult::Equal | OrderResult::LeftExhausted
    )
}

fn main() {
    let part1: i32 = shared::get_contents()
        .split("\n\n")
        .map(|lines| lines.split_once("\n").unwrap())
        .map(|(a, b)| {
            (
                serde_json::from_str::<Packet>(a).unwrap(),
                serde_json::from_str::<Packet>(b).unwrap(),
            )
        })
        .enumerate()
        .filter(|(_, (a, b))| are_ordered(a, b))
        .map(|(i, _)| i as i32 + 1)
        .sum();

    println!("{}", part1);
}
