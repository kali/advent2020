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
                .combinations(2)
                .all(|a| a[0] + a[1] != msg[pos])
        })
        .unwrap();
    let invalid = msg[pos];
    dbg!(invalid);
    let range = (0..pos)
        .combinations(2)
        .find(|a| msg[a[0]..=a[1]].iter().sum::<usize>() == invalid)
        .unwrap();
    let range = &msg[range[0]..=range[1]];
    dbg!(range.iter().min().unwrap() + range.iter().max().unwrap());
}
