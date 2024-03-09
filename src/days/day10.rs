use itertools::Itertools;

enum Pipes {
    V,
    H,
    NE,
    NW,
    SW,
    SE,
}

impl Pipes {
    fn parse(c: u8) -> Option<Pipes> {
        match c {
            b'|' => Some(Pipes::V),
            b'-' => Some(Pipes::H),
            b'L' => Some(Pipes::NE),
            b'J' => Some(Pipes::NW),
            b'7' => Some(Pipes::SW),
            b'F' => Some(Pipes::SE),
            c => None,
        }
    }

    fn connects(&self, start: usize, position: usize, row_len: usize) -> bool {
        match self {
            Pipes::V => start.abs_diff(position) == row_len,
            Pipes::H => start.abs_diff(position) == 1,
            Pipes::NE => start == position + 1 || start + row_len == position,
            Pipes::NW => start + 1 == position || start + row_len == position,
            Pipes::SE => start == position + 1 || start == position + row_len,
            Pipes::SW => start + 1 == position || start == position + row_len,
        }
    }

    fn next(&self, start: usize, position: usize, row_len: usize) -> usize {
        match self {
            Pipes::V if position > start => position + row_len,
            Pipes::V if position < start => position - row_len,
            Pipes::H if position > start => position + 1,
            Pipes::H if position < start => position - 1,
            Pipes::NE if position + 1 == start => position - row_len,
            Pipes::NE if position == start + row_len => position + 1,
            Pipes::NW if position == start + 1 => position - row_len,
            Pipes::NW if position == start + row_len => position - 1,
            Pipes::SE if position + 1 == start => position + row_len,
            Pipes::SE if position + row_len == start => position + 1,
            Pipes::SW if position == start + 1 => position + row_len,
            Pipes::SW if position + row_len == start => position - 1,

            _ => unreachable!("Why me?"),
        }
    }
}

pub fn day10() {
    let (result1, result2) = solve(include_bytes!("../../day10.txt"));

    println!(
        "DAY 10\nSolution 1: {:#?}\nSolution 2: {:#?}",
        result1, result2
    );
}

pub fn solve(map: &[u8]) -> (usize, usize) {
    let row_len = map
        .iter()
        .enumerate()
        .try_fold(false, |acc, (ix, p)| match acc {
            false => Ok(p == &10 || p == &13),
            true if p != &10 && p != &13 => Err(ix),
            true => Ok(true),
        })
        .unwrap_err();

    let start = map.iter().position(|p| p == &b'S').unwrap();

    let result1 = (0..)
        .try_fold(
            [
                (start, start - 1),
                (start, start + 1),
                (start, start + row_len),
                (start, start - row_len),
            ]
            .into_iter()
            .filter_map(|(s, d)| Pipes::parse(map[d]).and_then(|o| Some((s, d, o))))
            .filter(|(s, d, o)| o.connects(*s, *d, row_len))
            .map(|(s, d, _)| (s, d))
            .collect::<Vec<_>>(),
            |acc, v| {
                if v > 0 && acc[0].0 == acc[1].0 {
                    Err(v)
                } else {
                    Ok(acc
                        .into_iter()
                        .map(|(s, n)| (n, Pipes::parse(map[n]).unwrap().next(s, n, row_len)))
                        .collect_vec())
                }
            },
        )
        .unwrap_err();

    (result1, result1)
}
