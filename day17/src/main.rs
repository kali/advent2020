use std::collections::{HashMap, HashSet};

fn neigborhood(
    (x, y, z, w): (isize, isize, isize, isize),
    p2: bool,
) -> impl Iterator<Item = (isize, isize, isize, isize)> {
    (w - 1..=w + 1)
        .filter(move |w| p2 || *w == 0)
        .flat_map(move |w1| {
            (x - 1..=x + 1).flat_map(move |x1| {
                (y - 1..=y + 1).flat_map(move |y1| (z - 1..=z + 1).map(move |z1| (x1, y1, z1, w1)))
            })
        })
        .filter(move |co| co != &(x, y, z, w))
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    dbg!(run(&input));
}

fn run(input: &str) -> (usize, usize) {
    let input: HashSet<(isize, isize, isize, isize)> = input
        .lines()
        .enumerate()
        .flat_map(move |(y, l)| {
            l.chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some((x as isize, y as isize, 0, 0))
                } else {
                    None
                }
            })
        })
        .collect();
    let mut res = [false, true].iter().map(|part2| {
        let mut state = input.clone();
        for _ in 0..6 {
            let mut counters = HashMap::<_, usize>::new();
            for coords in state.iter().flat_map(|co| neigborhood(*co, *part2)) {
                *counters.entry(coords).or_default() += 1;
            }
            state = counters
                .into_iter()
                .filter(|(k, v)| (state.contains(&k) && *v == 2) || *v == 3)
                .map(|e| e.0)
                .collect();
        }
        state.len()
    });
    (res.next().unwrap(), res.next().unwrap())
}

#[test]
fn t0() {
    assert_eq!(run(".#.\n..#\n###"), (112, 848));
}
