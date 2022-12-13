mod shared;
use nom::{
    branch::alt,
    bytes::complete::take_while,
    character::complete::char,
    combinator::{cut, map},
    error::{context, ContextError, ParseError},
    multi::separated_list0,
    number::complete::double,
    sequence::{preceded, terminated},
    IResult,
};

#[derive(PartialEq, Debug)]
enum PacketData {
    Number(f64),
    List(Vec<PacketData>),
}

impl PacketData {
    fn new(n: f64) -> PacketData {
        PacketData::Number(n)
    }

    fn news(ns: &[f64]) -> PacketData {
        PacketData::List(Vec::from_iter(ns.iter().map(|n| PacketData::new(*n))))
    }
}

impl Default for PacketData {
    fn default() -> Self {
        PacketData::List(Vec::default())
    }
}

impl PacketData {
    fn right_order(&self, other: &Self) -> bool {
        match (self, other) {
            (PacketData::Number(n), PacketData::Number(m)) => n <= m,
            (PacketData::List(ns), PacketData::List(ms)) => {
                ns.iter().zip(ms.iter()).all(|(a, b)| {
                    dbg!((a, b));
                    PacketData::right_order(a, b)
                })
            }
            (PacketData::List(_), PacketData::Number(m)) => {
                self.right_order(&PacketData::news(&[*m]))
            }
            (PacketData::Number(n), PacketData::List(_)) => {
                other.right_order(&PacketData::news(&[*n]))
            }
        }
    }
}
fn sp<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let chars = " \t\r\n";

    take_while(move |c| chars.contains(c))(i)
}

fn packet_data_value<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, PacketData, E> {
    preceded(
        sp,
        alt((
            map(packet_data_list, PacketData::List),
            map(double, PacketData::Number),
        )),
    )(i)
}

fn packet_data_list<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, Vec<PacketData>, E> {
    context(
        "array",
        preceded(
            char('['),
            cut(terminated(
                separated_list0(preceded(sp, char(',')), packet_data_value),
                preceded(sp, char(']')),
            )),
        ),
    )(i)
}

fn packet_data<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, PacketData, E> {
    map(packet_data_list, PacketData::List)(i)
}

fn main() {
    let part1: i32 = shared::get_contents()
        .split("\n\n")
        .map(|lines| lines.split_once("\n").unwrap())
        .map(|(a, b)| {
            (
                packet_data::<_>(a).unwrap().1,
                packet_data::<_>(b).unwrap().1,
            )
        })
        .enumerate()
        .filter(|(_, (a, b))| a.right_order(b))
        .map(|(i, _)| i as i32)
        .sum();
}
