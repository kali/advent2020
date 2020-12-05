fn main() {
    let mut ids = std::fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|id| {
            id.trim().bytes().fold(0u16, |id, b| {
                (id << 1) | (b == b'B' || b == b'R') as usize as u16
            })
        })
        .collect::<Vec<u16>>();
    ids.sort();
    dbg!(ids.last().unwrap());
    for i in 0..ids.len() - 1 {
        if ids[i+1] - ids[i] == 2 {
            dbg!(ids[i] + 1);
        }
    }
}
