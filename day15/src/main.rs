use std::collections::HashMap;

fn main() {
    let bootstrap = std::fs::read_to_string("input")
        .unwrap()
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    dbg!(part_1(&*bootstrap, 2020));
    dbg!(part_1(&*bootstrap, 30000000));
}

fn part_1(bootstrap: &[usize], nth: usize) -> usize {
    let mut last = HashMap::<usize, Vec<usize>>::new();
    let mut current = 0usize;
    for i in 0..nth {
        current = if i < bootstrap.len() {
            bootstrap[i]
        } else if last[&current].len() > 1 {
            let h = &last[&current];
            h[h.len() - 1] - h[h.len() - 2]
        } else {
            0
        };
        last.entry(current).or_default().push(i);
    }
    current
}

#[test]
fn t0() {
    assert_eq!(part_1(&[0, 3, 6], 1), 0);
    assert_eq!(part_1(&[0, 3, 6], 2), 3);
    assert_eq!(part_1(&[0, 3, 6], 3), 6);
    assert_eq!(part_1(&[0, 3, 6], 4), 0);
    assert_eq!(part_1(&[0, 3, 6], 5), 3);
}
