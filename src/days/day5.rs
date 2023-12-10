use itertools::Itertools;

pub fn day5() {
    let result1 = include_str!("../../day5.txt")
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
        .fold(vec![], |p, v| {
            if !p.is_empty() {
                p.into_iter()
                    .map(|x| {
                        (
                            x,
                            v.iter()
                                .find(|r| x >= r[1] && x < r[1] + r[2])
                                .map(|vv| vv[0] - vv[1]),
                        )
                    })
                    .map(|(x, vv)| x + vv.unwrap_or(0))
                    .collect()
            } else {
                v.into_iter().nth(0).unwrap()
            }
        })
        .into_iter()
        .min()
        .unwrap();

    println!("DAY 1\nSolution 1: {}", result1);
}
