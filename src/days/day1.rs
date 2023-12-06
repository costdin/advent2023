use std::ops::ControlFlow;

pub fn day1() {
    let (result1, result2) = include_str!("../../day1.txt")
        .lines()
        .map(|l| {
            (
                find_first_number(l.chars(), true, false) * 10
                    + find_first_number(l.chars().rev(), true, true),
                find_first_number(l.chars(), false, false) * 10
                    + find_first_number(l.chars().rev(), false, true),
            )
        })
        .fold((0, 0), |(s1, s2), (n1, n2)| (s1 + n1, s2 + n2));

    println!("DAY 5\nSolution 1: {}\nSolution 2: {}", result1, result2);
}

fn find_first_number(mut s: impl Iterator<Item = char>, only_num: bool, rev: bool) -> u32 {
    match s.try_fold(37, |acc, c| fold(acc, c, only_num, rev)) {
        ControlFlow::Break(n) => n,
        _ => unreachable!("noooo"),
    }
}

fn fold(state: u32, digit: char, only_num: bool, rev: bool) -> ControlFlow<u32, u32> {
    match if digit.is_numeric() {
        (digit as u8 - b'0') as u32
    } else if only_num {
        37
    } else if rev {
        match (state - 10, digit) {
            (0, 'z') => 0,
            (1, 'o') => 1,
            (2, 't') => 2,
            (3, 't') => 3,
            (4, 'f') => 4,
            (5, 'f') => 5,
            (6, 's') => 6,
            (7, 's') => 7,
            (8, 'e') => 8,
            (9, 'n') => 9,

            (0 | 7 | 12 | 19, 'v') => 25,
            (0 | 7 | 13 | 19, 'e') => 22,
            (0 | 7 | 12 | 13 | 19, 'n') => 11,
            (1, 'i') => 19,
            (1 | 21, 'e') => 23,
            (4 | 18, 'r') => 21,
            (4 | 18, 'w') => 12,

            (10, 'e') => 17,
            (10 | 15, 'i') => 15,
            (11, 'h') => 13,
            (11, 'u') => 24,
            (11, 'e') => 10,
            (12, 'r') => 21,
            (13, 'v') => 20,
            (14, 'o') => 14,
            (16, 'g') => 27,
            (17, 'i') => 18,
            (20, 'i') => 16,
            (22, 'h') => 26,

            (_, 'o') => 28,
            (_, 'e') => 29,
            (_, 'r') => 21,
            (_, 'x') => 30,
            (_, 'n') => 31,
            (_, 't') => 32,

            _ => 37,
        }
    } else {
        match (state - 10, digit) {
            (0, 'o') => 0,
            (1, 'e') => 1,
            (2, 'o') => 2,
            (3, 'e') => 3,
            (4, 'r') => 4,
            (5, 'e') => 5,
            (6, 'x') => 6,
            (7, 'n') => 7,
            (8, 't') => 8,
            (9, 'e') => 9,

            (1 | 26 | 9, 'i') => 29,
            (10 | 3 | 15 | 7 | 25, 'i') => 27,
            (10, 'r') => 10,
            (11 | 21, 'n') => 11,
            (11, 'u') => 14,
            (12, 'r') => 23,
            (13, 'e') => 13,
            (14, 'v') => 15,
            (15, 'v') => 26,
            (16, 'e') => 17,
            (17, 'g') => 28,
            (18, 'h') => 18,
            (19, 'n') => 19,
            (20, 'e') => 20,
            (22, 'w') => 12,
            (22, 'h') => 22,
            (23, 'o') => 21,
            (23, 'i') => 24,
            (24, 'i') => 16,
            (24, 'e') => 25,

            (_, 'z') => 30,
            (_, 'o') => 31,
            (_, 't') => 32,
            (_, 'f') => 33,
            (_, 's') => 34,
            (_, 'e') => 35,
            (_, 'n') => 36,

            _ => 37,
        }
    } {
        n @ 0..=9 => ControlFlow::Break(n),
        n => ControlFlow::Continue(n),
    }
}
