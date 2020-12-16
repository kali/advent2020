fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    run(&*input);
}

fn run(input: &str) {
    let fields: Vec<(String, Vec<(usize, usize)>)> = input
        .split("\n\n")
        .nth(0)
        .unwrap()
        .lines()
        .map(|l| {
            let mut tokens = l.split(": ");
            let klass = tokens.next().unwrap();
            let ranges = tokens
                .next()
                .unwrap()
                .split(" or ")
                .map(|r| {
                    let mut tokens = r.split("-").map(|s| s.parse::<usize>().unwrap());
                    (tokens.next().unwrap(), tokens.next().unwrap())
                })
                .collect();
            (klass.to_string(), ranges)
        })
        .collect();
    let tickets: Vec<Vec<usize>> = input
        .split("\n\n")
        .nth(2)
        .unwrap()
        .split(['\n'].as_ref())
        .skip(1)
        .filter(|s| s.len() > 0)
        .map(|s| s.split(",").map(|s| s.parse::<usize>().unwrap()).collect())
        .collect();
    let p1 = tickets
        .iter()
        .flatten()
        .filter(|n| {
            !fields
                .iter()
                .flat_map(|v| v.1.iter())
                .any(|r| r.0 <= **n && **n <= r.1)
        })
        .sum::<usize>();
    dbg!(p1);
    let valid: Vec<&[usize]> = tickets
        .iter()
        .map(|v| &**v)
        .filter(|t| {
            t.iter().all(|n| {
                fields
                    .iter()
                    .any(|p| p.1.iter().any(|r| r.0 <= *n && *n <= r.1))
            })
        })
        .collect();
    let fields_len = fields.len();
    let fields: Vec<(String, Vec<usize>)> = fields
        .into_iter()
        .map(|(name, ranges)| {
            (
                name,
                (0..fields_len)
                    .filter(|&i| {
                        valid
                            .iter()
                            .all(|t| ranges.iter().any(|r| r.0 <= t[i] && t[i] <= r.1))
                    })
                    .collect(),
            )
        })
        .collect();
    let valid: Vec<usize> = input
        .split("\n\n")
        .nth(1)
        .unwrap()
        .lines()
        .nth(1)
        .unwrap()
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let solution = locate_fields(&mut vec![None; fields_len], &fields).unwrap();
    let p2 = (0..fields_len)
        .filter(|f| fields[*f].0.starts_with("departure "))
        .map(|f| valid[solution.iter().position(|p| *p == f).unwrap()])
        .product::<usize>();
    dbg!(p2);
}

fn locate_fields(
    fixed: &mut [Option<usize>],
    fields: &[(String, Vec<usize>)],
) -> Option<Vec<usize>> {
    if let Some(done) = fixed.iter().copied().collect() {
        return Some(done);
    }
    let locating_ix = fields
        .iter()
        .enumerate()
        .filter(|(ix, _)| !fixed.contains(&Some(*ix)))
        .min_by_key(|(_, (_, locations))| {
            locations
                .iter()
                .filter(|loc| !fixed.contains(&Some(**loc)))
                .count()
        })
        .unwrap()
        .0;
    let locating = &fields[locating_ix];
    for place in locating.1.iter() {
        if fixed[*place].is_some() {
            continue;
        }
        fixed[*place] = Some(locating_ix);
        if let Some(r) = locate_fields(fixed, fields) {
            return Some(r);
        }
        fixed[*place] = None;
    }
    None
}

#[test]
fn t0() {
    let t = r#"class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9"#;
    run(t);
}
