fn main() {
    let mut adaptors: Vec<usize> = std::fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect();
    adaptors.push(0);
    adaptors.push(adaptors.iter().max().unwrap() + 3);
    adaptors.sort();
    let diff1 = adaptors.windows(2).filter(|p| p[1] - p[0] == 1).count();
    let diff3 = adaptors.windows(2).filter(|p| p[1] - p[0] == 3).count();
    dbg!(diff1 * diff3);
    let mut ways = vec![0u64; *adaptors.iter().max().unwrap() + 1];
    ways[0] = 1;
    for adaptor in adaptors {
        for source in adaptor.saturating_sub(3)..adaptor {
            ways[adaptor] += ways[source];
        }
    }
    dbg!(ways.last().unwrap());
}
