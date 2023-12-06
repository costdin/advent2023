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
        _ => unreachable!("Let's not spend too much time handling errors"),
    }
}

fn fold(state: u32, digit: char, only_num: bool, rev: bool) -> ControlFlow<u32, u32> {
    match if digit.is_numeric() {
        (digit as u8 - b'0') as u32
    } else if only_num {
        37
    } else {
        MAP[if rev { 728 } else { 0 }
            + ((state - 10) * 26) as usize
            + (digit as u8 - b'a') as usize]
    } {
        n @ 0..=9 => ControlFlow::Break(n),
        n => ControlFlow::Continue(n),
    }
}

const MAP: [u32; 1456] = [
    37, 37, 37, 37, 35, 33, 37, 37, 37, 37, 37, 37, 37, 36, 0, 37, 37, 37, 34, 32, 37, 37, 37, 37,
    37, 30, 37, 37, 37, 37, 1, 33, 37, 37, 29, 37, 37, 37, 37, 36, 31, 37, 37, 37, 34, 32, 37, 37,
    37, 37, 37, 30, 37, 37, 37, 37, 35, 33, 37, 37, 37, 37, 37, 37, 37, 36, 2, 37, 37, 37, 34, 32,
    37, 37, 37, 37, 37, 30, 37, 37, 37, 37, 3, 33, 37, 37, 27, 37, 37, 37, 37, 36, 31, 37, 37, 37,
    34, 32, 37, 37, 37, 37, 37, 30, 37, 37, 37, 37, 35, 33, 37, 37, 37, 37, 37, 37, 37, 36, 31, 37,
    37, 4, 34, 32, 37, 37, 37, 37, 37, 30, 37, 37, 37, 37, 5, 33, 37, 37, 37, 37, 37, 37, 37, 36,
    31, 37, 37, 37, 34, 32, 37, 37, 37, 37, 37, 30, 37, 37, 37, 37, 35, 33, 37, 37, 37, 37, 37, 37,
    37, 36, 31, 37, 37, 37, 34, 32, 37, 37, 37, 6, 37, 30, 37, 37, 37, 37, 35, 33, 37, 37, 27, 37,
    37, 37, 37, 7, 31, 37, 37, 37, 34, 32, 37, 37, 37, 37, 37, 30, 37, 37, 37, 37, 35, 33, 37, 37,
    37, 37, 37, 37, 37, 36, 31, 37, 37, 37, 34, 8, 37, 37, 37, 37, 37, 30, 37, 37, 37, 37, 9, 33,
    37, 37, 29, 37, 37, 37, 37, 36, 31, 37, 37, 37, 34, 32, 37, 37, 37, 37, 37, 30, 37, 37, 37, 37,
    35, 33, 37, 37, 27, 37, 37, 37, 37, 36, 31, 37, 37, 10, 34, 32, 37, 37, 37, 37, 37, 30, 37, 37,
    37, 37, 35, 33, 37, 37, 37, 37, 37, 37, 37, 11, 31, 37, 37, 37, 34, 32, 14, 37, 37, 37, 37, 30,
    37, 37, 37, 37, 35, 33, 37, 37, 37, 37, 37, 37, 37, 36, 31, 37, 37, 23, 34, 32, 37, 37, 37, 37,
    37, 30, 37, 37, 37, 37, 13, 33, 37, 37, 37, 37, 37, 37, 37, 36, 31, 37, 37, 37, 34, 32, 37, 37,
    37, 37, 37, 30, 37, 37, 37, 37, 35, 33, 37, 37, 37, 37, 37, 37, 37, 36, 31, 37, 37, 37, 34, 32,
    37, 15, 37, 37, 37, 30, 37, 37, 37, 37, 35, 33, 37, 37, 27, 37, 37, 37, 37, 36, 31, 37, 37, 37,
    34, 32, 37, 26, 37, 37, 37, 30, 37, 37, 37, 37, 17, 33, 37, 37, 37, 37, 37, 37, 37, 36, 31, 37,
    37, 37, 34, 32, 37, 37, 37, 37, 37, 30, 37, 37, 37, 37, 35, 33, 28, 37, 37, 37, 37, 37, 37, 36,
    31, 37, 37, 37, 34, 32, 37, 37, 37, 37, 37, 30, 37, 37, 37, 37, 35, 33, 37, 18, 37, 37, 37, 37,
    37, 36, 31, 37, 37, 37, 34, 32, 37, 37, 37, 37, 37, 30, 37, 37, 37, 37, 35, 33, 37, 37, 37, 37,
    37, 37, 37, 19, 31, 37, 37, 37, 34, 32, 37, 37, 37, 37, 37, 30, 37, 37, 37, 37, 20, 33, 37, 37,
    37, 37, 37, 37, 37, 36, 31, 37, 37, 37, 34, 32, 37, 37, 37, 37, 37, 30, 37, 37, 37, 37, 35, 33,
    37, 37, 37, 37, 37, 37, 37, 11, 31, 37, 37, 37, 34, 32, 37, 37, 37, 37, 37, 30, 37, 37, 37, 37,
    35, 33, 37, 22, 37, 37, 37, 37, 37, 36, 31, 37, 37, 37, 34, 32, 37, 37, 12, 37, 37, 30, 37, 37,
    37, 37, 35, 33, 37, 37, 24, 37, 37, 37, 37, 36, 21, 37, 37, 37, 34, 32, 37, 37, 37, 37, 37, 30,
    37, 37, 37, 37, 25, 33, 37, 37, 16, 37, 37, 37, 37, 36, 31, 37, 37, 37, 34, 32, 37, 37, 37, 37,
    37, 30, 37, 37, 37, 37, 35, 33, 37, 37, 27, 37, 37, 37, 37, 36, 31, 37, 37, 37, 34, 32, 37, 37,
    37, 37, 37, 30, 37, 37, 37, 37, 35, 33, 37, 37, 29, 37, 37, 37, 37, 36, 31, 37, 37, 37, 34, 32,
    37, 37, 37, 37, 37, 30, 37, 37, 37, 37, 35, 33, 37, 37, 37, 37, 37, 37, 37, 36, 31, 37, 37, 37,
    34, 32, 37, 37, 37, 37, 37, 30, 37, 37, 37, 37, 22, 37, 37, 37, 37, 37, 37, 37, 37, 11, 28, 37,
    37, 21, 37, 32, 37, 25, 37, 30, 37, 0, 37, 37, 37, 37, 23, 37, 37, 37, 19, 37, 37, 37, 37, 31,
    1, 37, 37, 21, 37, 32, 37, 37, 37, 30, 37, 37, 37, 37, 37, 37, 29, 37, 37, 37, 37, 37, 37, 37,
    37, 31, 28, 37, 37, 21, 37, 2, 37, 37, 37, 30, 37, 37, 37, 37, 37, 37, 29, 37, 37, 37, 37, 37,
    37, 37, 37, 31, 28, 37, 37, 21, 37, 3, 37, 37, 37, 30, 37, 37, 37, 37, 37, 37, 29, 4, 37, 37,
    37, 37, 37, 37, 37, 31, 28, 37, 37, 21, 37, 32, 37, 37, 12, 30, 37, 37, 37, 37, 37, 37, 29, 5,
    37, 37, 37, 37, 37, 37, 37, 31, 28, 37, 37, 21, 37, 32, 37, 37, 37, 30, 37, 37, 37, 37, 37, 37,
    29, 37, 37, 37, 37, 37, 37, 37, 37, 31, 28, 37, 37, 21, 6, 32, 37, 37, 37, 30, 37, 37, 37, 37,
    37, 37, 22, 37, 37, 37, 37, 37, 37, 37, 37, 11, 28, 37, 37, 21, 7, 32, 37, 25, 37, 30, 37, 37,
    37, 37, 37, 37, 8, 37, 37, 37, 37, 37, 37, 37, 37, 31, 28, 37, 37, 21, 37, 32, 37, 37, 37, 30,
    37, 37, 37, 37, 37, 37, 29, 37, 37, 37, 37, 37, 37, 37, 37, 9, 28, 37, 37, 21, 37, 32, 37, 37,
    37, 30, 37, 37, 37, 37, 37, 37, 17, 37, 37, 37, 15, 37, 37, 37, 37, 31, 28, 37, 37, 21, 37, 32,
    37, 37, 37, 30, 37, 37, 37, 37, 37, 37, 10, 37, 37, 13, 37, 37, 37, 37, 37, 31, 28, 37, 37, 21,
    37, 32, 24, 37, 37, 30, 37, 37, 37, 37, 37, 37, 29, 37, 37, 37, 37, 37, 37, 37, 37, 11, 28, 37,
    37, 21, 37, 32, 37, 25, 37, 30, 37, 37, 37, 37, 37, 37, 22, 37, 37, 37, 37, 37, 37, 37, 37, 11,
    28, 37, 37, 21, 37, 32, 37, 20, 37, 30, 37, 37, 37, 37, 37, 37, 29, 37, 37, 37, 37, 37, 37, 37,
    37, 31, 14, 37, 37, 21, 37, 32, 37, 37, 37, 30, 37, 37, 37, 37, 37, 37, 29, 37, 37, 37, 15, 37,
    37, 37, 37, 31, 28, 37, 37, 21, 37, 32, 37, 37, 37, 30, 37, 37, 37, 37, 37, 37, 29, 37, 27, 37,
    37, 37, 37, 37, 37, 31, 28, 37, 37, 21, 37, 32, 37, 37, 37, 30, 37, 37, 37, 37, 37, 37, 29, 37,
    37, 37, 18, 37, 37, 37, 37, 31, 28, 37, 37, 21, 37, 32, 37, 37, 37, 30, 37, 37, 37, 37, 37, 37,
    29, 37, 37, 37, 37, 37, 37, 37, 37, 31, 28, 37, 37, 21, 37, 32, 37, 37, 12, 30, 37, 37, 37, 37,
    37, 37, 22, 37, 37, 37, 37, 37, 37, 37, 37, 11, 28, 37, 37, 21, 37, 32, 37, 25, 37, 30, 37, 37,
    37, 37, 37, 37, 29, 37, 37, 37, 16, 37, 37, 37, 37, 31, 28, 37, 37, 21, 37, 32, 37, 37, 37, 30,
    37, 37, 37, 37, 37, 37, 23, 37, 37, 37, 37, 37, 37, 37, 37, 31, 28, 37, 37, 21, 37, 32, 37, 37,
    37, 30, 37, 37, 37, 37, 37, 37, 29, 37, 37, 26, 37, 37, 37, 37, 37, 31, 28, 37, 37, 21, 37, 32,
    37, 37, 37, 30, 37, 37, 37, 37, 37, 37, 29, 37, 37, 37, 37, 37, 37, 37, 37, 31, 28, 37, 37, 21,
    37, 32, 37, 37, 37, 30, 37, 37, 37, 37, 37, 37, 29, 37, 37, 37, 37, 37, 37, 37, 37, 31, 28, 37,
    37, 21, 37, 32, 37, 37, 37, 30, 37, 37, 37, 37, 37, 37, 29, 37, 37, 37, 37, 37, 37, 37, 37, 31,
    28, 37, 37, 21, 37, 32, 37, 37, 37, 30, 37, 37, 37, 37, 37, 37, 29, 37, 37, 37, 37, 37, 37, 37,
    37, 31, 28, 37, 37, 21, 37, 32, 37, 37, 37, 30, 37, 37, 37, 37, 37, 37, 29, 37, 37, 37, 37, 37,
    37, 37, 37, 31, 28, 37, 37, 21, 37, 32, 37, 37, 37, 30, 37, 37,
];
