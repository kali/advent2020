use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Eq, PartialEq, Hash)]
struct Tile(Vec<Vec<bool>>, u64);

impl Tile {
    fn at(&self, mut x: usize, mut y: usize, transform: u8) -> bool {
        if (transform & 0x1) == 0x01 {
            std::mem::swap(&mut x, &mut y);
        }
        if (transform & 0x2) == 0x02 {
            x = self.0[0].len() - 1 - x;
        }
        if (transform & 0x4) == 0x04 {
            y = self.0.len() - 1 - y;
        }
        self.0[y][x]
    }

    fn height(&self, transform: u8) -> usize {
        if transform & 0x01 == 0x01 {
            self.0[0].len()
        } else {
            self.0.len()
        }
    }

    fn width(&self, transform: u8) -> usize {
        if transform & 0x01 == 0x01 {
            self.0.len()
        } else {
            self.0[0].len()
        }
    }

    fn edges(&self) -> [Vec<bool>; 4] {
        [self.top(0), self.left(0), self.bottom(0), self.right(0)]
    }

    fn row(&self, y: usize, transform: u8) -> Vec<bool> {
        (0..self.width(transform))
            .map(|x| self.at(x, y, transform))
            .collect()
    }

    fn col(&self, x: usize, transform: u8) -> Vec<bool> {
        (0..self.height(transform))
            .map(|y| self.at(x, y, transform))
            .collect()
    }

    fn top(&self, transform: u8) -> Vec<bool> {
        self.row(0, transform)
    }

    fn bottom(&self, transform: u8) -> Vec<bool> {
        self.row(self.height(transform) - 1, transform)
    }

    fn left(&self, transform: u8) -> Vec<bool> {
        self.col(0, transform)
    }

    fn right(&self, transform: u8) -> Vec<bool> {
        self.col(self.width(transform) - 1, transform)
    }
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height(0) {
            for x in 0..self.width(0) {
                write!(f, "{}", if self.at(x, y, 0) { b'#' } else { b'.' } as char)?;
            }
            if y == 0 {
                write!(f, " {}", self.1)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

fn parse(input: &str) -> Vec<Tile> {
    input
        .trim()
        .split("\n\n")
        .map(|tile| {
            let mut lines = tile.trim().lines();
            let id = lines
                .next()
                .unwrap()
                .trim()
                .split_whitespace()
                .nth(1)
                .unwrap()
                .trim_end_matches(":")
                .parse::<u64>()
                .unwrap();
            let lines = lines
                .map(|s| s.bytes().map(|b| b == b'#').collect())
                .collect::<Vec<_>>();
            Tile(lines, id)
        })
        .collect()
}

fn find_corners(tiles: &[Tile]) -> Vec<&Tile> {
    let mut edge_index: HashMap<Vec<bool>, Vec<&Tile>> = HashMap::new();
    for tile in tiles {
        for edge in tile.edges().iter() {
            let mut reversed = edge.clone();
            reversed.reverse();
            let min = edge.min(&reversed);
            edge_index.entry(min.clone()).or_default().push(tile);
        }
    }
    let uniques_edges: Vec<(Vec<bool>, &Tile)> = edge_index
        .iter()
        .filter(|(_, v)| v.len() == 1)
        .map(|(k, v)| (k.clone(), v[0]))
        .collect();
    let mut unique_per_tile: HashMap<&Tile, usize> = HashMap::new();
    for edge in uniques_edges {
        *unique_per_tile.entry(edge.1).or_default() += 1;
    }
    let corners: Vec<&Tile> = unique_per_tile
        .iter()
        .filter(|(_, v)| **v == 2)
        .map(|(id, _)| *id)
        .collect();
    assert_eq!(corners.len(), 4);
    corners
}

fn find_right<'a>(tiles: &'a [Tile], from: (&Tile, u8)) -> Option<(&'a Tile, u8)> {
    tiles
        .iter()
        .filter(|tile| tile != &from.0)
        .flat_map(|tile| (0u8..8).map(move |tr| (tile, tr)))
        .find(|(tile, tr)| tile.left(*tr) == from.0.right(from.1))
}

fn find_down<'a>(tiles: &'a [Tile], from: (&Tile, u8)) -> Option<(&'a Tile, u8)> {
    tiles
        .iter()
        .filter(|tile| tile != &from.0)
        .flat_map(|tile| (0u8..8).map(move |tr| (tile, tr)))
        .find(|(tile, tr)| tile.top(*tr) == from.0.bottom(from.1))
}

fn solve<'t>(tiles: &'t [Tile], corner: &'t Tile) -> Vec<Vec<(&'t Tile, u8)>> {
    let corner_tr = (0..8)
        .find(|&tr| {
            find_down(tiles, (corner, tr)).is_some() && find_right(tiles, (corner, tr)).is_some()
        })
        .unwrap();
    let mut solved = vec![vec![(corner, corner_tr)]];
    while let Some(down) = find_down(&tiles, solved.last().unwrap()[0]) {
        solved.push(vec![down]);
    }
    for row in solved.iter_mut() {
        while let Some(right) = find_right(&tiles, *row.last().unwrap()) {
            row.push(right)
        }
    }
    solved
}

fn build_map(solution: &[Vec<(&Tile, u8)>]) -> Vec<Vec<bool>> {
    let h = solution.len() * 8;
    let w = solution[0].len() * 8;
    let mut map = vec![];
    for y in 0..h {
        let (ty, dy) = (y / 8, y % 8);
        let mut line = vec![];
        for x in 0..w {
            let (tx, dx) = (x / 8, x % 8);
            let tile = solution[ty][tx];
            line.push(tile.0.at(dx + 1, dy + 1, tile.1));
        }
        map.push(line);
    }
    map
}

fn monster() -> Tile {
    let input = r#"..................#.
#....##....##....###
.#..#..#..#..#..#..."#;
    let lines = input
        .lines()
        .map(|s| s.bytes().map(|b| b == b'#').collect())
        .collect::<Vec<_>>();
    Tile(lines, 0)
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    dbg!(run(&input));
}

fn run(input: &str) -> (u64, usize) {
    let tiles = parse(&input);
    let corners = find_corners(&tiles);
    let part1 = corners.iter().map(|t| t.1).product::<u64>();
    let corner = corners[0];
    let solved = solve(&tiles, corner);
    let mut map = Tile(build_map(&solved), 0);
    let monster = monster();
    let mut monsters = vec![];
    for tr in 0..8 {
        let mh = monster.height(tr);
        let mw = monster.width(tr);
        for x in 0..map.width(0) - mw {
            'm: for y in 0..map.height(0) - mh {
                for mx in 0..mw {
                    for my in 0..mh {
                        if monster.at(mx, my, tr) && !map.at(x + mx, y + my, 0) {
                            continue 'm;
                        }
                    }
                }
                monsters.push((x, y, tr));
            }
        }
    }
    for (x, y, tr) in monsters {
        let mh = monster.height(tr);
        let mw = monster.width(tr);
        for mx in 0..mw {
            for my in 0..mh {
                if monster.at(mx, my, tr) {
                    map.0[y + my][x + mx] = false;
                }
            }
        }
    }
    let part2 = map
        .0
        .iter()
        .map(|l| l.iter().filter(|x| **x).count())
        .sum::<usize>();
    (part1, part2)
}

#[test]
fn t() {
    let input = std::fs::read_to_string("test").unwrap();
    assert_eq!(run(&input), (20899048083289u64, 273));
}
