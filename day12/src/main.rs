use num::complex::Complex as C;

fn main() {
    let (z, r, i) = (C::new(0, 0), C::new(1, 0), C::i());
    let ops: Vec<_> = std::fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|line| {
            let n = &line[1..].parse::<i32>().unwrap();
            match line.as_bytes()[0] as char {
                'R' => ((-i).powi(n / 90), z, z),
                'L' => (i.powi(n / 90), z, z),
                'N' => (r, i * n, z),
                'E' => (r, r * n, z),
                'W' => (r, -r * n, z),
                'S' => (r, -i * n, z),
                'F' => (r, z, r * n),
                _ => panic!(),
            }
        })
        .collect();

    for &p in &[0, 1] {
        let pos = ops
            .iter()
            .fold((z, r + (9 * r + i) * p), |(s, d), (r, t, f)| {
                (s + f * d + t * (1 - p), d * r + t * p)
            })
            .0;
        dbg!(pos.im.abs() + pos.re.abs());
    }
}
