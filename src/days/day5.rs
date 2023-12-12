use itertools::Itertools;

pub fn day5() {
    let (result1, result2) = match include_str!("../../day5.txt")
        .lines()
        .group_by(|l| !l.is_empty())
        .into_iter()
        .filter_map(|g| if g.0 { Some(g.1) } else { None })
        .map(|g| {
            g.map(|l| {
                l.split(' ')
                    .filter(|s| !s.is_empty() && s.as_bytes()[0] >= 48 && s.as_bytes()[0] <= 57)
                    .map(|s| {
                        s.as_bytes()
                            .iter()
                            .fold(0u64, |a, n| a as u64 * 10 + *n as u64 - 48)
                    })
                    .collect::<Vec<u64>>()
            })
            .filter(|c| !c.is_empty())
            .collect::<Vec<_>>()
        })
        .fold((vec![], vec![]), |(p, pr), v| {
            if !p.is_empty() {
                (
                    p.into_iter()
                        .map(|x| {
                            v.iter()
                                .find(|r| x >= r[1] && x < r[1] + r[2])
                                .map(|vv| x - vv[1] + vv[0])
                                .unwrap_or(x)
                        })
                        .collect(),
                    mama_take_this_job_from_me_i_dont_deserve_it_anymore(&pr, &v),
                )
            } else {
                (
                    v[0].clone(),
                    v[0].chunks(2)
                        .map(|a| [a[0], a[1]])
                        .collect::<Vec<[u64; 2]>>(),
                )
            }
        }) {
        (v1, v2) => (
            v1.into_iter().min().unwrap(),
            v2.into_iter().map(|r| r[0]).sorted().nth(0).unwrap(),
        ),
    };

    println!("DAY 5\nSolution 1: {}\nSolution 2: {}", result1, result2);
}

fn mama_take_this_job_from_me_i_dont_deserve_it_anymore(
    s: &[[u64; 2]],
    v: &[Vec<u64>],
) -> Vec<[u64; 2]> {
    s.into_iter()
        .flat_map(|x| {
            (0..)
                .try_fold((Vec::<[u64; 2]>::new(), *x), |(acc, c), _| match c {
                    [s, l] => match v.iter().find(|r| s >= r[1] && s < (r[1] + r[2])) {
                        Some(r) if s - r[1] + l <= r[2] => Err(acc
                            .into_iter()
                            .chain([[s - r[1] + r[0], l]])
                            .collect::<Vec<[u64; 2]>>()),
                        Some(r) => Ok((
                            acc.into_iter()
                                .chain([[s - r[1] + r[0], r[2] + r[1] - s]])
                                .collect::<Vec<[u64; 2]>>(),
                            [r[1] + r[2], l + s - r[2] - r[1]],
                        )),
                        None => match v
                            .iter()
                            .filter(|r| (s + l) >= r[1] && s < r[1])
                            .map(|r| r[1])
                            .sorted()
                            .nth(0)
                        {
                            None => Err(vec![c]),
                            Some(r) => Ok((
                                acc.into_iter()
                                    .chain([[s, r - s]])
                                    .collect::<Vec<[u64; 2]>>(),
                                [r, l + s - r],
                            )),
                        },
                    },
                })
                .unwrap_err()
        })
        .collect()
}
