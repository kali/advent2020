use bloom::BloomFilter;
use bloom::ASMS;
use itertools::Itertools;

const CHUNK: usize = 10000usize;

#[derive(Default)]
struct Rope(Vec<(BloomFilter, Vec<u32>)>);

impl Rope {
    fn new_bl() -> BloomFilter {
        BloomFilter::with_rate(0.001, (CHUNK + CHUNK / 8) as u32)
    }

    fn rebalance(&mut self) {
        let new = self
            .iter()
            .chunks(CHUNK)
            .into_iter()
            .map(|c| {
                let mut bl = Self::new_bl();
                let v = c
                    .inspect(|x| {
                        bl.insert(x);
                    })
                    .collect();
                (bl, v)
            })
            .collect();
        *self = Rope(new)
    }

    fn iter(&self) -> impl Iterator<Item = u32> + '_ + Clone {
        self.0.iter().flat_map(|v| v.1.iter()).copied()
    }

    fn loc(&self, value: u32) -> (usize, usize) {
        for (vix, v) in self.0.iter().enumerate() {
            if v.0.contains(&value) {
                if let Some(ix) = v.1.iter().position(|&x| x == value) {
                    return (vix, ix);
                }
            }
        }
        unreachable!()
    }

    fn push_back(&mut self, i: u32) {
        if self.0.is_empty() || self.0.last().as_mut().unwrap().1.len() > 2000 {
            self.0.push((Self::new_bl(), vec![]));
        }
        self.0.last_mut().as_mut().unwrap().0.insert(&i);
        self.0.last_mut().as_mut().unwrap().1.push(i);
    }

    fn pop_front(&mut self) -> u32 {
        while self.0[0].1.is_empty() {
            self.0.remove(0);
        }
        self.0[0].1.remove(0)
    }

    fn insert_after(&mut self, v: u32, buf: &[u32]) {
        let (vix, ix) = self.loc(v);
        for x in buf.iter().rev() {
            self.0[vix].0.insert(x);
            self.0[vix].1.insert(ix + 1, *x)
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    dbg!(run(&input, false));
    dbg!(run(&input, true));
}

fn run(input: &str, part2: bool) -> usize {
    let mut input: Vec<u32> = input.bytes().map(|b| (b - b'0') as u32).collect();
    if part2 {
        input.extend((input.len() + 1) as u32..=1_000_000);
        assert_eq!(input.len(), 1_000_000);
    }
    let mut data = Rope::default();
    input.iter().for_each(|&x| data.push_back(x));
    data.rebalance();
    let mut buffer = vec![];
    let rounds = if part2 { 10_000_000 } else { 100 };
    for r in 0..rounds {
        for i in 0..4 {
            buffer.push(data.pop_front());
        }
        let mut dst_id = buffer[0];
        while buffer.contains(&dst_id) {
            dst_id = if dst_id == 1 { input.len() as u32 } else { dst_id - 1 };
        }
        data.insert_after(dst_id, &buffer[1..]);
        data.push_back(buffer[0]);
        buffer.clear();
        if r % 100_000 == 0 {
            data.rebalance();
        }
    }
    if !part2 {
        data.iter()
            .cycle()
            .skip_while(|&x| x != 1)
            .skip(1)
            .take(8)
            .fold(0usize, |acc, i| acc * 10 + i as usize)
    } else {
        data.iter()
            .cycle()
            .skip_while(|&x| x != 1)
            .skip(1)
            .take(2)
            .fold(1usize, |acc, i| acc * i as usize)
    }
}

#[test]
fn t0() {
    assert_eq!(run("389125467", false), 67384529);
}
