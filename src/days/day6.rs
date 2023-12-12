use crate::days::parse_integer;

pub fn day6() {
    let (result1, result2) = match include_str!("../../day6.txt")
        .lines()
        .map(|l| {
            l.split(' ')
                .filter(|s| !s.is_empty() && s.as_bytes()[0] >= 48 && s.as_bytes()[0] <= 57)
                .map(parse_integer)
                .collect::<Vec<_>>()
        })
        .try_fold(None, |prev: Option<Vec<_>>, curr| match prev {
            None => Ok(Some(curr)),
            Some(p) => Err(p.into_iter().zip(curr)),
        })
        .unwrap_err()
        .into_iter()
        .fold::<(Box<dyn Iterator<Item = (u128, u128)>>, _), _>(
            (Box::new(vec![].into_iter()), (0, 0)),
            |(acc, last), n| {
                (
                    Box::new(acc.into_iter().chain([n])),
                    (
                        last.0 * 10u128.pow((n.0 as f64).log10().floor() as u32 + 1) + n.0,
                        last.1 * 10u128.pow((n.1 as f64).log10().floor() as u32 + 1) + n.1,
                    ),
                )
            },
        ) {
        (v, n) => v.chain([n]).collect::<Vec<_>>(),
    }
    .iter()
    .map(|(t, d)| (t, f64::sqrt((t * t - 4 * d) as f64)))
    .map(|(t, n)| (t, n, n as u128))
    .map(|(t, nf, ni)| match (t, nf, ni) {
        (_, f, i) if i as f64 == f => i - 1,
        (t, _, i) if i % 2 != t % 2 => i,
        (_, _, i) => i + 1,
    })
    .fold((1, 1), |(acc, l), n| (acc * l, n));

    println!("DAY 6\nSolution 1: {}\nSolution 2: {}", result1, result2);
}
