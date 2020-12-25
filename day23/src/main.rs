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
    let mut follower = vec![0; input.len() + 1];
    for i in 0..input.len() - 1 {
        follower[input[i] as usize] = input[i + 1];
    }
    follower[*input.last().unwrap() as usize] = input[0];
    let rounds = if part2 { 10_000_000 } else { 100 };
    let mut current: u32 = input[0];
    let mut buffer: Vec<u32> = vec![];
    for _ in 0..rounds {
        buffer.push(current);
        buffer.push(follower[current as usize]);
        buffer.push(follower[follower[current as usize] as usize]);
        buffer.push(follower[follower[follower[current as usize] as usize] as usize]);
        let mut dst_id = buffer[0];
        while buffer.contains(&dst_id) {
            dst_id = if dst_id == 1 {
                input.len() as u32
            } else {
                dst_id - 1
            };
        }
        follower[current as usize] = follower[buffer[3] as usize];
        let after = follower[dst_id as usize];
        follower[dst_id as usize] = buffer[1];
        follower[buffer[3] as usize] = after;
        current = follower[current as usize];
        buffer.clear();
    }
    if !part2 {
        let mut c = 1u32;
        let mut v = 0u32;
        for _ in 0..8 {
            c = follower[c as usize];
            v = v * 10 + c
        }
        v as usize
    } else {
        let a = follower[1] as usize;
        let b = follower[a] as usize;
        a * b
    }
}

#[test]
fn t0() {
    assert_eq!(run("389125467", false), 67384529);
}
