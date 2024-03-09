pub fn day9() {
    let (result1, result2) = include_str!("../../day9.txt")
        .lines()
        .map(|l| {
            l.split(' ')
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|s| {
            (0..)
                .try_fold((0, 0, s), |(acc, acc_back, l), i| {
                    // TODO: Improve this, we don't need to iterate twice, but I don't want to refactor this now
                    if l.iter().all(|n| n == &0) {
                        Err((acc, acc_back))
                    } else {
                        Ok((
                            acc + *l.last().unwrap(),
                            if i % 2 == 0 {
                                acc_back + *l.first().unwrap()
                            } else {
                                acc_back - *l.first().unwrap()
                            },
                            l.iter()
                                .zip(l.iter().skip(1))
                                .map(|(a, b)| *b - *a)
                                .collect(),
                        ))
                    }
                })
                .expect_err("Please don't crash!")
        })
        .fold((0, 0), |(forwards, backwards), (f, b)| {
            (forwards + f, backwards + b)
        });

    println!(
        "DAY 9\nSolution 1: {:#?}\nSolution 2: {:#?}",
        result1, result2
    );
}
