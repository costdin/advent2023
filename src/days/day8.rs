use std::collections::HashMap;

pub fn day8() {
    let (result1, result2) = match match include_str!("../../day8.txt")
        .lines()
        .filter(|l| !l.is_empty())
        .fold::<(_, Box<dyn Iterator<Item = (&str, (&str, &str))>>), _>(
            (vec![], Box::new(vec![].into_iter())),
            |(i, l), r| match (i, r) {
                (i, _) if i.len() == 0 => (r.as_bytes().iter().collect(), l),
                (i, r) => (i, Box::new(l.chain([(&r[0..3], (&r[7..10], &r[12..15]))]))),
            },
        ) {
        (o, l) => (o, l.collect::<HashMap<_, _>>()),
    } {
        (o, m) => (
            (0..)
                .try_fold("AAA", |p, i| match (p, o[i % o.len()]) {
                    ("ZZZ", _) => Err(i),
                    (_, 76) => Ok(m[p].0),
                    (_, 82) => Ok(m[p].1),
                    _ => unreachable!("How did we get here?"),
                })
                .unwrap_err(),
            m.keys()
                .filter(|k| k.as_bytes()[2] == 65)
                .map(|p| {
                    (0..)
                        .try_fold(*p, |p, i| match (p, o[i % o.len()]) {
                            (s, _) if s.as_bytes()[2] == 90 => Err(i),
                            (_, 76) => Ok(m[p].0),
                            (_, 82) => Ok(m[p].1),
                            _ => unreachable!("How did we get here?"),
                        })
                        .unwrap_err()
                })
                .fold(0, |acc, n| {
                    if acc == 0 {
                        n as u64
                    } else {
                        lcm(acc, n as u64)
                    }
                }),
        ),
    };

    println!(
        "DAY 8\nSolution 1: {:#?}\nSolution 2: {:#?}",
        result1, result2
    );
}

fn lcm(n: u64, m: u64) -> u64 {
    n * m / gcd(n, m)
}

fn gcd(n: u64, m: u64) -> u64 {
    if n == m {
        n
    } else if m > n {
        gcd(m, n)
    } else {
        gcd(n - m, m)
    }
}
