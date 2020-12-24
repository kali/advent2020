use std::collections::{HashMap, HashSet};

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let reached: Vec<(isize, isize)> = input
        .lines()
        .map(|mut l| {
            let mut reached = (0, 0);
            while l.len() > 0 {
                let (len, ne, se) = match (l.chars().nth(0).unwrap(), l.chars().nth(1)) {
                    ('e', _) => (1, 1, 1),
                    ('w', _) => (1, -1, -1),
                    ('n', Some('e')) => (2, 1, 0),
                    ('s', Some('e')) => (2, 0, 1),
                    ('n', Some('w')) => (2, 0, -1),
                    ('s', Some('w')) => (2, -1, 0),
                    _ => unreachable!()
                };
                reached = (reached.0 + ne, reached.1 + se);
                l = &l[len..];
            }
            reached
        })
        .collect();
    let mut state = HashMap::<(isize, isize), bool>::new();
    for reached in reached {
        *state.entry(reached).or_default() ^= true;
    }
    let mut state: HashSet<_> = state.iter().filter(|(_, v)| **v).map(|(k, _)| *k).collect();
    dbg!(state.len());
    for _ in 0..100 {
        let mut counters = HashMap::<(isize, isize), usize>::new();
        for (ne, se) in state.iter() {
            for (dne, dse) in &[(1, 0), (-1, 0), (0, 1), (0, -1), (-1, -1), (1, 1)] {
                *counters.entry((ne + dne, se + dse)).or_default() += 1;
            }
        }
        let mut next = HashSet::new();
        for (&poi, &neighbors) in counters.iter() {
            let black = state.contains(&poi);
            if (black && neighbors >= 1 && neighbors <= 2) || (!black && neighbors == 2) {
                next.insert(poi);
            }
        }
        state = next;
    }
    dbg!(state.len());
}
