use itertools::Itertools;

fn main() {
    for len in &[2, 3] {
        let winner = std::fs::read_to_string("input")
            .unwrap()
            .trim()
            .lines()
            .map(|s| s.parse::<usize>().unwrap())
            .combinations(*len)
            .find(|comb| comb.iter().sum::<usize>() == 2020)
            .unwrap();
        dbg!(winner.iter().product::<usize>());
    }
}
