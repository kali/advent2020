use std::collections::HashMap;

#[derive(Debug)]
enum Line {
    Mask { bytes: Vec<u8> },
    Store { addr: u64, value: u64 },
}

fn parse(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|l| {
            if l.starts_with("mask = ") {
                let bytes = l.split_whitespace().last().unwrap().bytes().collect();
                Line::Mask { bytes }
            } else {
                let addr = l
                    .split(['[', ']'].as_ref())
                    .nth(1)
                    .unwrap()
                    .parse::<u64>()
                    .unwrap();
                let value = l.split_whitespace().last().unwrap().parse::<u64>().unwrap();
                Line::Store { addr, value }
            }
        })
        .collect()
}

fn part_1(lines: &[Line]) -> u64 {
    let mut mem = HashMap::<u64, u64>::new();
    let mut current_zero = 0;
    let mut current_ones = 0;
    for line in lines {
        match line {
            Line::Mask { bytes } => {
                let (zeroes, ones) = bytes.iter().fold((0u64, 0u64), |(zero, ones), bit| {
                    (
                        (zero << 1) | (*bit == b'0') as u64,
                        (ones << 1) | (*bit == b'1') as u64,
                    )
                });
                current_zero = zeroes;
                current_ones = ones;
            }
            Line::Store { addr, value } => {
                mem.insert(*addr, (value | current_ones) & !current_zero);
            }
        }
    }
    mem.values().sum::<u64>()
}

fn part_2(lines: &[Line]) -> u64 {
    let mut mem = HashMap::<u64, u64>::new();
    let mut mask: &[u8] = &[];
    fn store(mem: &mut HashMap<u64, u64>, msb: u64, mask: &[u8], add: u64, v: u64) {
        if mask.len() == 0 {
            mem.insert(msb, v);
        } else {
            if mask[0] == b'0' {
                let add_bit = (add >> (mask.len() - 1)) & 1;
                store(mem, msb << 1 | add_bit, &mask[1..], add, v);
            } else if mask[0] == b'1' {
                store(mem, msb << 1 | 1, &mask[1..], add, v);
            } else {
                store(mem, msb << 1, &mask[1..], add, v);
                store(mem, msb << 1 | 1, &mask[1..], add, v);
            }
        }
    }
    for line in lines {
        match line {
            Line::Mask { bytes } => mask = &*bytes,
            Line::Store { addr, value } => store(&mut mem, 0, mask, *addr, *value),
        }
    }
    mem.values().sum::<u64>()
}

fn main() {
    let lines = parse(&std::fs::read_to_string("input").unwrap());
    dbg!(part_1(&lines));
    dbg!(part_2(&lines));
}

#[test]
fn t0() {
    let lines = parse(
        "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0",
    );
    assert_eq!(part_1(&lines), 165);
}

#[test]
fn t1() {
    let lines = parse(
        "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1 ",
    );
    assert_eq!(part_2(&lines), 208);
}
