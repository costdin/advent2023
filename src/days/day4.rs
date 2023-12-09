use std::iter;

pub fn day4() {
    let (result1, result2, _) = include_str!("../../day4.txt")
        .lines()
        .map(|l| l.split_once(':').unwrap().1)
        .filter_map(|l| l.split_once('|'))
        .map(|(wn, pn)| {
            pn.split(" ")
                .filter(|s| !s.is_empty())
                .map(|s| s.as_bytes().iter().fold(0, |a, n| a * 10 + n - 48))
                .fold(
                    (
                        wn.split(" ")
                            .filter(|s| !s.is_empty())
                            .map(|s| s.as_bytes().iter().fold(0, |a, n| a * 10 + n - 48))
                            .collect::<Vec<_>>(),
                        0,
                        0,
                    ),
                    |(w, s, c), n| {
                        if w.contains(&n) {
                            (w, if s == 0 { 1 } else { s * 2 }, c + 1)
                        } else {
                            (w, s, c)
                        }
                    },
                )
        })
        .fold((0, 0, vec![]), |(r1, r2, a), (_, p, c)| {
            (
                r1 + p,
                r2 + *a.iter().next().unwrap_or(&0) + 1,
                if c > 0 && a.len() >= c + 1 {
                    iter::repeat(1 + *a.iter().next().unwrap_or(&0))
                        .take(c + 1)
                        .chain(iter::repeat(0))
                        .zip(a.iter())
                        .map(|(a, b)| a + b)
                        .skip(1)
                        .collect()
                } else if c > 0 {
                    iter::repeat(1 + *a.iter().next().unwrap_or(&0))
                        .take(c + 1)
                        .zip(a.into_iter().chain(iter::repeat(0)))
                        .map(|(a, b)| a + b)
                        .skip(1)
                        .collect()
                } else {
                    a.into_iter().skip(1).collect()
                },
            )
        });
    println!("DAY 1\nSolution 1: {}\nSolution 2: {}", result1, result2);
}
