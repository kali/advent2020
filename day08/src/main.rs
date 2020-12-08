use std::collections::HashSet;

fn run(lines: &[String]) -> Result<isize, isize> {
    let mut pc = 0;
    let mut acc = 0isize;
    let mut seen = HashSet::new();
    loop {
        let tokens = lines[pc].split_whitespace().collect::<Vec<&str>>();
        match tokens[0] {
            "nop" => pc += 1,
            "acc" => {
                acc += tokens[1].parse::<isize>().unwrap();
                pc += 1
            }
            "jmp" => {
                pc = (pc as isize + tokens[1].parse::<isize>().unwrap()) as usize;
            }
            _ => panic!(),
        }
        if seen.contains(&pc) {
            break Err(acc);
        }
        if pc == lines.len() {
            return Ok(acc)
        }
        seen.insert(pc);
    }
}

fn main() {
    let mut lines = std::fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect::<Vec<_>>();
    dbg!(run(&lines));
    for i in 0..lines.len() {
        let saved = lines[i].clone();
        if lines[i].starts_with("nop") {
            lines[i] = lines[i].replace("nop", "jmp");
        } else if lines[i].starts_with("jmp") {
            lines[i] = lines[i].replace("jmp", "nop");
        } else {
            continue
        }
        if let Ok(acc) = run(&lines) {
            dbg!(acc);
            return;
        }
        lines[i] = saved;
    }
}
