mod shared;
use serde_json::Value;

fn right_order(a: &Value, other: &Value) -> bool {
    match (self, other) {
        (Value::Number(n), Value::Number(m)) => n <= m,
        (PacketData::List(ns), PacketData::List(ms)) => ns
            .iter()
            .zip(ms.iter())
            .all(|(a, b)| PacketData::right_order(a, b)),
        (PacketData::List(_), PacketData::Number(m)) => {
            self.right_order(&PacketData::List(vec![PacketData::Number(*m)]))
        }
        (PacketData::Number(n), PacketData::List(_)) => {
            other.right_order(&PacketData::List(vec![PacketData::Number(*n)]))
        }
    }
}

fn main() {
    let part1: i32 = shared::get_contents()
        .split("\n\n")
        .map(|lines| lines.split_once("\n").unwrap())
        .map(|(a, b)| {
            dbg!(a);
            (
                serde_json::from_str(a).unwrap(),
                serde_json::from_str(b).unwrap(),
            )
        })
        .enumerate()
        .filter(|(_, (a, b))| a.right_order(b))
        .map(|(i, _)| i as i32)
        .sum();

    println!("{}", part1);
}
