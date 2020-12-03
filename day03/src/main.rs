fn trees(lines: &[Vec<bool>], hstride: usize, vstride: usize) -> usize {
    let width = lines[0].len();
    let (mut x, mut y, mut trees) = (0, 0, 0);
    loop {
        trees += lines[y][x] as usize;
        x += hstride;
        y += vstride;
        if x >= width {
            x -= width;
        }
        if y >= lines.len() {
            return trees;
        }
    }
}

fn main() {
    let lines: Vec<Vec<bool>> = std::fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|s| s.trim().bytes().map(|c| c == b'#').collect())
        .collect();
    dbg!(trees(&lines, 3, 1));
    let part_2 = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(h, v)| trees(&lines, *h, *v))
        .product::<usize>();
    dbg!(part_2);
}
