use std::collections::{HashSet, VecDeque};

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let [mine, theirs] = parse(&input);
    dbg!(game1(mine.clone(), theirs.clone()));
    dbg!(game2(mine.clone(), theirs.clone()));
}

fn parse(input: &str) -> [VecDeque<usize>; 2] {
    let mut decks = input.split("\n\n").map(|p| {
        p.lines()
            .skip(1)
            .map(|l| l.parse::<usize>().unwrap())
            .collect()
    });
    let mine = decks.next().unwrap();
    let theirs = decks.next().unwrap();
    [mine, theirs]
}

fn game1(mut mine: VecDeque<usize>, mut theirs: VecDeque<usize>) -> usize {
    while !mine.is_empty() && !theirs.is_empty() {
        let (me, th) = (mine.pop_front().unwrap(), theirs.pop_front().unwrap());
        if me < th {
            theirs.push_back(th);
            theirs.push_back(me);
        } else {
            mine.push_back(me);
            mine.push_back(th);
        }
    }
    let winner = if mine.is_empty() { theirs } else { mine };
    winner
        .iter()
        .enumerate()
        .map(|(ix, v)| (winner.len() - ix) * v)
        .sum::<usize>()
}

fn game2(mut mine: VecDeque<usize>, mut theirs: VecDeque<usize>) -> usize {
    let winner = if game2_rec(&mut mine, &mut theirs) {
        theirs
    } else {
        mine
    };
    winner
        .iter()
        .enumerate()
        .map(|(ix, v)| (winner.len() - ix) * v)
        .sum::<usize>()
}

fn game2_rec(mine: &mut VecDeque<usize>, theirs: &mut VecDeque<usize>) -> bool {
    let mut history = HashSet::new();
    loop {
        if !history.insert(mine.clone()) || theirs.len() == 0 {
            return false;
        }
        if mine.len() == 0 {
            return true;
        }
        let (me, th) = (mine.pop_front().unwrap(), theirs.pop_front().unwrap());
        let ilost = if me <= mine.len() && th <= theirs.len() {
            let mut mine = mine.iter().take(me).copied().collect();
            let mut theirs = theirs.iter().take(th).copied().collect();
            game2_rec(&mut mine, &mut theirs)
        } else {
            th > me
        };
        if ilost {
            theirs.push_back(th);
            theirs.push_back(me);
        } else {
            mine.push_back(me);
            mine.push_back(th);
        };
    }
}

#[test]
fn t0() {
    assert_eq!(run(&std::fs::read_to_string("test").unwrap()), 306);
}
