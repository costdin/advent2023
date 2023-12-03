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

enum State {
    Result(u16),
    Z,
    E,
    R,
    O,
    N,
    T,
    W,
    R3,
    H,
    E3,
    F,
    O4,
    U,
    I,
    V,
    S,
    I6,
    E7,
    V7,
    E77,
    E8,
    I8,
    G,
    H8,
    N9,
    I9,
    N99,
    N7,
    X,
    E33,
    None,
}

impl Into<ControlFlow<u16, State>> for State {
    fn into(self) -> ControlFlow<u16, State> { 
        match self {
            State::Result(n) => ControlFlow::Break(n),
            s => ControlFlow::Continue(s)
        }
    }
}

fn find_first_number(mut s: impl Iterator<Item = char>, only_num: bool, rev: bool) -> u16 {
    match s.try_fold(State::None, |acc, c| fold(acc, c, only_num, rev)) {
        ControlFlow::Break(n) => n,
        _ => unreachable!("noooo"),
    }
}

fn fold(state: State, digit: char, only_num: bool, rev: bool) -> ControlFlow<u16, State> {
    if digit.is_numeric() {
        ControlFlow::Break((digit as u8 - b'0') as u16)
    } else if only_num {
        ControlFlow::Continue(State::None)
    } else if rev {
        match (state, digit) {
            (State::E, 'z') => ControlFlow::Break(0),
            (State::N, 'o') => ControlFlow::Break(1),
            (State::W, 't') => ControlFlow::Break(2),
            (State::H, 't') => ControlFlow::Break(3),
            (State::O4, 'f') => ControlFlow::Break(4),
            (State::I, 'f') => ControlFlow::Break(5),
            (State::I6, 's') => ControlFlow::Break(6),
            (State::E77, 's') => ControlFlow::Break(7),
            (State::I8, 'e') => ControlFlow::Break(8),
            (State::I9, 'n') => ControlFlow::Break(9),

            (State::U, 'o') => State::O4.into(),
            (State::E7, 'v') => State::V7.into(),
            (State::O | State::O4, 'r') => State::R.into(),
            (State::R, 'e') => State::E.into(),
            (State::E | State::E3 | State::E7 | State::E77, 'e') => State::E33.into(),
            (State::E | State::E3 | State::E33 | State::E7 | State::E77, 'n') => State::N.into(),
            (State::N7 | State::N, 'e') => State::E7.into(),
            (State::V7, 'e') => State::E77.into(),
            (State::O | State::O4, 'w') => State::W.into(),
            (State::E33, 'r') => State::R.into(),
            (State::R, 'h') => State::H.into(),
            (State::R, 'u') => State::U.into(),
            (State::E | State::E3 | State::E33 | State::E77, 'v') => State::V.into(),
            (State::V | State::V7, 'i') => State::I.into(),
            (State::X, 'i') => State::I6.into(),
            (State::T, 'h') => State::H8.into(),
            (State::H8, 'g') => State::G.into(),
            (State::G, 'i') => State::I8.into(),
            (State::N, 'i') => State::I9.into(),

            (_, 'o') => State::O.into(),
            (_, 'e') => State::E3.into(),
            (_, 'r') => State::R.into(),
            (_, 'x') => State::X.into(),
            (_, 'n') => State::N7.into(),
            (_, 't') => State::T.into(),

            _ => State::None.into(),
        }
    } else {
        match (state, digit) {
            (State::R, 'o') => ControlFlow::Break(0),
            (State::N, 'e') => ControlFlow::Break(1),
            (State::W, 'o') => ControlFlow::Break(2),
            (State::E3, 'e') => ControlFlow::Break(3),
            (State::U, 'r') => ControlFlow::Break(4),
            (State::V, 'e') => ControlFlow::Break(5),
            (State::I6, 'x') => ControlFlow::Break(6),
            (State::E77, 'n') => ControlFlow::Break(7),
            (State::H8, 't') => ControlFlow::Break(8),
            (State::N99, 'e') => ControlFlow::Break(9),

            (State::Z, 'e') =>  State::E.into(),
            (State::E, 'r') => State::R.into(),
            (State::F, 'o') => State::O4.into(),
            (State::I9, 'n') => State::N99.into(),
            (State::O | State::O4, 'n') => State::N.into(),
            (State::T, 'w') => State::W.into(),
            (State::T, 'h') => State::H.into(),
            (State::H, 'r') => State::R3.into(),
            (State::R3, 'e') => State::E3.into(),
            (State::O4, 'u') => State::U.into(),
            (State::F, 'i') => State::I.into(),
            (State::I, 'v') => State::V.into(),
            (State::S, 'i') => State::I6.into(),
            (State::S, 'e') => State::E7.into(),
            (State::E7, 'v') => State::V7.into(),
            (State::V7, 'e') => State::E77.into(),
            (State::E | State::E3 | State::E7 | State::E77 | State::E8, 'i') => State::I8.into(),
            (State::I8, 'g') => State::G.into(),
            (State::G, 'h') => State::H8.into(),
            (State::N | State::N9 | State::N99, 'i') => State::I9.into(),

            (_, 'z') => State::Z.into(),
            (_, 'o') => State::O.into(),
            (_, 't') => State::T.into(),
            (_, 'f') => State::F.into(),
            (_, 's') => State::S.into(),
            (_, 'e') => State::E8.into(),
            (_, 'n') => State::N9.into(),

            _ => State::None.into(),
        }
    }
}
