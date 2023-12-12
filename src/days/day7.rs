use crate::days::parse_integer;
use itertools::Itertools;
use std::cmp::Ordering::{Greater, Less};

const THE_THING: [u8; 70] = [
    0, 1, 2, 3, 4, 5, 6, 7, 58, 59, 60, 61, 62, 63, 64, 12, 98, 31, 58, 41, 53, 10, 15, 78, 9, 11,
    45, 1, 11, 15, 8, 10, 4, 4, 8, 1, 2, 3, 4, 5, 6, 7, 8, 58, 59, 60, 61, 62, 63, 64, 13, 98, 31,
    58, 41, 53, 10, 15, 78, 0, 12, 45, 1, 11, 15, 8, 11, 4, 4, 9,
];

pub fn day7() {
    let (result1, result2) = match include_str!("../../day7.txt")
        .lines()
        .filter_map(|l| l.split_once(' '))
        .map(|(h, b)| (h, parse_integer::<u64>(b)))
        .map(|(h, b)| {
            (
                (
                    13u64.pow(5)
                        * h.as_bytes()
                            .into_iter()
                            .sorted()
                            .group_by(|c| *c)
                            .into_iter()
                            .map(|(_, g)| 3u64.pow(g.count() as u32))
                            .sum::<u64>()
                        + h.as_bytes()
                            .into_iter()
                            .fold(0, |acc, b| acc * 13 + THE_THING[*b as usize - 50] as u64),
                    b,
                ),
                (
                    13u64.pow(10)
                        * h.as_bytes()
                            .into_iter()
                            .sorted()
                            .group_by(|c| *c)
                            .into_iter()
                            .map(|(k, g)| (k, g.count() as u32))
                            .sorted_by(|(k1, g1), (k2, g2)| match (k1, k2) {
                                (74, 74) => g2.cmp(&g1),
                                (74, _) => Less,
                                (_, 74) => Greater,
                                _ => g2.cmp(&g1),
                            })
                            .fold((0, None), |(sum, jokers), (card, count)| {
                                match (card, count) {
                                    (_, 5) => (3u64.pow(5), None),
                                    (74, _) => (sum, Some(count)),
                                    _ => (sum + 3u64.pow(count + jokers.unwrap_or(0)), None),
                                }
                            })
                            .0
                        + h.as_bytes()
                            .into_iter()
                            .fold(0, |acc, b| acc * 100 + THE_THING[*b as usize - 15] as u64),
                    b,
                ),
            )
        })
        .unzip::<_, _, Vec<_>, Vec<_>>()
    {
        (v1, v2) => (
            v1.iter()
                .sorted()
                .zip(1..)
                .map(|((_, b), ix)| ix * b)
                .sum::<u64>(),
            v2.iter()
                .sorted()
                .zip(1..)
                .map(|((_, b), ix)| ix * b)
                .sum::<u64>(),
        ),
    };

    println!(
        "DAY 7\nSolution 1: {:#?}\nSolution 2: {:#?}",
        result1, result2
    );
}
