fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let pks: Vec<u32> = input.lines().map(|k| k.parse::<u32>().unwrap()).collect();
    let p1 = crack_symkey(pks[0], pks[1]);
    dbg!(p1);
}

fn crack_symkey(pka: u32, pkb: u32) -> u32 {
    let (cracked_sk, other_pk) = match crack_one(pka, pkb) {
        (Some(a), None) => (a, pkb),
        (None, Some(b)) => (b, pka),
        _ => panic!(),
    };
    (0..cracked_sk).fold(1, |v, _| ((other_pk as u64 * v as u64) % 20201227) as u32)
}

fn crack_one(pka: u32, pkb: u32) -> (Option<u32>, Option<u32>) {
    let mut v = 1;
    for sk in 0.. {
        if v == pka {
            return (Some(sk), None);
        }
        if v == pkb {
            return (None, Some(sk));
        }
        v = (7 * v) % 20201227;
    }
    unreachable!()
}

#[test]
fn test_crack() {
    assert_eq!(crack_one(5764801, 17807724), (Some(8), None));
    assert_eq!(crack_symkey(5764801, 17807724), 14897079);
}
