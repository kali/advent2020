use std::fmt::Debug;

#[derive(PartialEq, Clone)]
struct State(Vec<Vec<u8>>);

fn rules_p1(state: &State, col: usize, row: usize) -> u8 {
    let mut occupied = 0;
    let x_min = col.saturating_sub(1);
    let x_max = (col + 1).min(state.0[0].len() - 1);
    let y_min = row.saturating_sub(1);
    let y_max = (row + 1).min(state.0.len() - 1);
    for x in x_min..=x_max {
        for y in y_min..=y_max {
            occupied += ((x != col || y != row) && state.0[y][x] == b'#') as usize;
        }
    }
    if state.0[row][col] == b'L' && occupied == 0 {
        b'#'
    } else if occupied >= 4 {
        b'L'
    } else {
        state.0[row][col]
    }
}

fn rules_p2(state: &State, col: usize, row: usize) -> u8 {
    let mut occupied = 0;
    for dir in &[
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ] {
        for step in 1.. {
            let x = col as isize + dir.0 * step as isize;
            let y = row as isize + dir.1 * step as isize;
            if x < 0 || y < 0 || x >= state.0[0].len() as isize || y >= state.0.len() as isize {
                break;
            }
            if state.0[y as usize][x as usize] != b'.' {
                occupied += (state.0[y as usize][x as usize] == b'#') as usize;
                break;
            }
        }
    }
    if state.0[row][col] == b'L' && occupied == 0 {
        b'#'
    } else if occupied >= 5 {
        b'L'
    } else {
        state.0[row][col]
    }
}

impl State {
    fn next(&self, rules: &impl Fn(&State, usize, usize) -> u8) -> State {
        let mut state = self.0.clone();
        for row in 0..self.0.len() {
            for col in 0..self.0[0].len() {
                if state[row][col] == b'.' {
                    continue;
                };
                state[row][col] = rules(self, col, row)
            }
        }
        State(state)
    }

    fn fix_point(&self, rules: &impl Fn(&State, usize, usize) -> u8) -> State {
        let mut state = self.clone();
        loop {
            let next_state = state.next(rules);
            if state == next_state {
                break;
            }
            state = next_state;
        }
        state
    }

    fn count(&self) -> usize {
        self.0.iter().flatten().filter(|b| **b == b'#').count()
    }

    fn parse(input: &str) -> State {
        let seats = input
            .lines()
            .map(|s| s.as_bytes().to_vec())
            .collect::<Vec<Vec<u8>>>();
        State(seats.clone())
    }
}

impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.iter().try_for_each(|line| {
            writeln!(f, "{}", line.iter().map(|b| *b as char).collect::<String>())
        })
    }
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    dbg!(State::parse(&input).fix_point(&rules_p1).count());
    dbg!(State::parse(&input).fix_point(&rules_p2).count());
}

#[cfg(test)]
#[test]
fn test_1() {
    let input = State::parse(
        r#"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"#,
    );

    assert_eq!(
        input.next(&rules_p1),
        State::parse(
            "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##"
        )
    );
    assert_eq!(
        input.next(&rules_p1).next(&rules_p1),
        State::parse(
            "#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##"
        )
    );
    assert_eq!(input.fix_point(&rules_p1).count(), 37);
}

#[cfg(test)]
#[test]
fn test_2() {
    let input = State::parse(
        r#"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"#,
    );

    let state = input.next(&rules_p2);
    assert_eq!(
        state,
        State::parse(
            "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##"
        )
    );
    let state = state.next(&rules_p2);
    assert_eq!(
        state,
        State::parse(
            "#.LL.LL.L#
#LLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLLL.L
#.LLLLL.L#"
        )
    );
    let state = state.next(&rules_p2);
    assert_eq!(
        state,
        State::parse(
            "#.L#.##.L#
#L#####.LL
L.#.#..#..
##L#.##.##
#.##.#L.##
#.#####.#L
..#.#.....
LLL####LL#
#.L#####.L
#.L####.L#"
        )
    );
    assert_eq!(input.fix_point(&rules_p2).count(), 26);
}
