fn main() {
    let db = std::fs::read_to_string("input")
        .unwrap()
        .trim()
        .lines()
        .map(|line| {
            let mut tokens = line.split([':', '-', '"', ' ',].as_ref());
            let low = tokens.next().unwrap().parse::<usize>().unwrap();
            let high = tokens.next().unwrap().parse::<usize>().unwrap();
            let letter = tokens.next().unwrap().chars().next().unwrap();
            tokens.next().unwrap();
            let pwd = tokens.next().unwrap().to_string();
            (low, high, letter, pwd)
        })
        .collect::<Vec<_>>();
    let part_1 = db
        .iter()
        .filter(|&(low, high, letter, pwd)| {
            let count = pwd.chars().filter(|c| letter == c).count();
            &count >= low && &count <= high
        })
        .count();
    dbg!(part_1);
    let part_2 = db
        .iter()
        .filter(|&(low, high, letter, pwd)| {
            (pwd.chars().nth(low - 1).unwrap() == *letter)
                ^ (pwd.chars().nth(high - 1).unwrap() == *letter)
        })
        .count();
    dbg!(part_2);
}
