use itertools::Itertools;
use std::collections::HashMap;

pub fn day3() {
    let (result1, result2) = solve(include_bytes!("../../day3.txt"));

    println!("DAY 3\nSolution 1: {}\nSolution 2: {}", result1, result2);
}

fn solve(map: &[u8]) -> (u32, u32) {
    sub_solve(
        map.iter()
            .enumerate()
            .try_fold(0, |se, (ix, c)| {
                if se == 0 && c < &32 {
                    Ok(ix)
                } else if se != 0 && c >= &32 {
                    Err(ix)
                } else {
                    Ok(se)
                }
            })
            .unwrap_err(),
        map,
    )
}

fn sub_solve(len: usize, map: &[u8]) -> (u32, u32) {
    sub_sub_solve(
        map,
        map.iter()
            .enumerate()
            .filter(|(_, n)| n > &&32 && n < &&46 || n == &&47 || n > &&57)
            .flat_map(|(ix, n)| {
                (((ix.max(1) - 1)..=(ix + 1))
                    .chain((ix.max(len + 1) - (len + 1))..=(ix.max(len - 1) - (len - 1)))
                    .chain((ix + (len - 1))..=(ix + (len + 1))))
                .map(move |p| {
                    if *n == 42 {
                        (p * 2, ix)
                    } else {
                        (p * 2 + 1, 0)
                    }
                })
            })
            .collect::<HashMap<_, _>>(),
    )
}

fn sub_sub_solve(map: &[u8], sym: HashMap<usize, usize>) -> (u32, u32) {
    match map
        .iter()
        .enumerate()
        .fold::<(_, _, _, Box<dyn Iterator<Item = (usize, u32)>>), _>(
            (0u32, 0, 0u32, Box::new(vec![].into_iter())),
            |(sum, dc, curr, hm), (ix, c)| match (c, curr) {
                (d @ 48..=57, n) => (sum, dc + 1, n * 10 + *d as u32 - 48, hm),
                (_, 0) => (sum, 0, 0, hm),
                (_, n) => match ((ix - dc)..ix).filter_map(|i| sym.get(&(i * 2))).nth(0) {
                    Some(a) => (sum + n, 0, 0, Box::new(hm.chain([(*a, n)].into_iter()))),
                    None => match ((ix - dc)..ix).filter_map(|i| sym.get(&(i * 2 + 1))).nth(0) {
                        Some(_) => (sum + n, 0, 0, hm),
                        None => (sum, 0, 0, hm),
                    },
                },
            },
        ) {
        (b, _, _, e) => (
            b,
            e.sorted()
                .group_by(|(a, _)| *a)
                .into_iter()
                .map(|(_, g)| g.collect::<Vec<_>>())
                .filter(|v| v.len() == 2)
                .map(|v| v[0].1 * v[1].1)
                .sum(),
        ),
    }
}
