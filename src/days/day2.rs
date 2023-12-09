pub fn day2() {
    let (result1, result2) = include_str!("../../day2.txt")
        .lines()
        .map(|l| {
            l[5..].split(':').fold((None, None), |(g, _), ll| {
                if g.is_none() {
                    (
                        Some(ll.chars().fold(0, |a, n| a * 10 + n as u32 - 48)),
                        None,
                    )
                } else {
                    (
                        g,
                        Some(
                            ll.split(';')
                                .flat_map(|ll| ll.split(","))
                                .map(|ll| {
                                    ll[1..]
                                        .chars()
                                        .map(|c| c as u32)
                                        .try_fold((0, false), |(a, f), n| match (n, f) {
                                            (n @ 48..=57, false) => Ok((a * 10 + n - 48, false)),
                                            (32, false) => Ok((a, true)),
                                            (c, true) => Err((a, c)),
                                            _ => unreachable!("Why are we here?"),
                                        })
                                        .unwrap_err()
                                })
                                .fold((0, 0, 0), |(r, g, b), (n, c)| match c {
                                    98 => (r, g, b.max(n)),
                                    103 => (r, g.max(n), b),
                                    114 => (r.max(n), g, b),
                                    _ => unreachable!("Why are we here?"),
                                }),
                        ),
                    )
                }
            })
        })
        .map(|(o1, o2)| (o1.unwrap(), o2.unwrap()))
        .fold((0, 0), |(ga, pa), (game, (r, g, b))| {
            (
                ga + if r <= 12 && g <= 13 && b <= 14 {
                    game
                } else {
                    0
                },
                pa + r * g * b,
            )
        });

    println!("DAY 1\nSolution 1: {}\nSolution 2: {}", result1, result2);
}
