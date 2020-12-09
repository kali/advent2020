use itertools::Itertools;

fn main() {
    let msg = std::fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<usize>>();
    let pos = (25..)
        .find(|&pos| {
            msg[pos - 25..pos]
                .iter()
                .tuple_combinations()
                .all(|(a, b)| a + b != msg[pos])
        })
        .unwrap();
    let invalid = msg[pos];
    dbg!(invalid);
    let (a, b) = (0..pos)
        .tuple_combinations()
        .find(|&(a, b)| msg[a..=b].iter().sum::<usize>() == invalid)
        .unwrap();
    let range = &msg[a..=b];
    dbg!(range.iter().min().unwrap() + range.iter().max().unwrap());
}
