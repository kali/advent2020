fn parse_ids(input: &str) -> Vec<Option<usize>> {
    input
        .split(",")
        .map(|s| {
            if s == "x" {
                None
            } else {
                Some(s.parse::<usize>().unwrap())
            }
        })
        .collect()
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut input = input.lines();
    let time = input.next().unwrap().parse::<usize>().unwrap();
    let ids = parse_ids(input.next().unwrap());

    let first_start = ids
        .iter()
        .filter_map(|s| *s)
        .map(|id| (id - time % id, id))
        .min()
        .unwrap();

    dbg!(first_start.0 * first_start.1);
    dbg!(part_2(ids));
}

fn euclid(a: i64, b: i64) -> (i64, i64, i64) {
    let mut ruv = (a, 1, 0);
    let mut ruv1 = (b, 0, 1);
    while ruv1.0 != 0 {
        let q = ruv.0 / ruv1.0;
        let ruv2 = (ruv.0 - q * ruv1.0, ruv.1 - q * ruv1.1, ruv.2 - q * ruv1.2);
        ruv = ruv1;
        ruv1 = ruv2;
    }
    ruv
}

fn chinese_remainders_theorem(pairs: Vec<(u64, u64)>) -> u64 {
    let n = pairs.iter().map(|pair| pair.0).product::<u64>();
    pairs
        .iter()
        .map(|&(ni, ai)| {
            let ni1 = n / ni;
            let vi = euclid(ni as i64, ni1 as i64).2;
            let vi = (vi + ni as i64) as u64 % ni;
            (vi * ni1 * ai) % n
        })
        .sum::<u64>()
        % n
}

fn part_2(ids: Vec<Option<usize>>) -> u64 {
    let remainders: Vec<(u64, u64)> = ids
        .iter()
        .enumerate()
        .filter_map(|(ix, id)| id.map(|id| (id as _, ((1 + ix / id) * id - ix) as _)))
        .collect();
    chinese_remainders_theorem(remainders)
}

#[test]
fn t_euclid() {
    assert_eq!(euclid(120, 23), (1, -9, 47));
}

#[test]
fn t_chinese_remainders() {
    assert_eq!(chinese_remainders_theorem(vec!((3, 2), (5, 3), (7, 2))), 23);
}

#[test]
fn t_part_2_0() {
    assert_eq!(part_2(parse_ids("17,x,13,19")), 3417);
}

#[test]
fn t_part_2_1() {
    assert_eq!(part_2(parse_ids("7,13,x,x,59,x,31,19")), 1068781);
}
