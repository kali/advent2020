use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let part_1 = std::fs::read_to_string("input")
        .unwrap()
        .split("\n\n")
        .map(|g| {
            g.chars()
                .filter(|c| c.is_alphanumeric())
                .collect::<HashSet<char>>()
                .len()
        })
        .sum::<usize>();
    dbg!(part_1);
    let part_2 = std::fs::read_to_string("input")
        .unwrap()
        .split("\n\n")
        .map(|g| {
            let mut counters: HashMap<char, usize> = HashMap::new();
            for c in g.chars().filter(|c| c.is_alphanumeric()) {
                *counters.entry(c).or_default() += 1;
            }
            let len = g.lines().count();
            counters.values().filter(|v| **v == len).count()
        })
        .sum::<usize>();
    dbg!(part_2);
}
