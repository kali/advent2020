use std::collections::HashMap;

type Specs = HashMap<String, Vec<(String, usize)>>;

fn contains_at_least_one(specs: &Specs, haystack: &str, needle: &str) -> bool {
    haystack == needle
        || specs[haystack]
            .iter()
            .any(|hay| contains_at_least_one(specs, &hay.0, needle))
}

fn bags_inside(specs: &Specs, bag: &str) -> usize {
    specs[bag]
        .iter()
        .map(|hay| hay.1 * (1 + bags_inside(specs, &hay.0)))
        .sum::<usize>()
}

fn main() {
    let specs: Specs = std::fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|line| {
            let mut tokens = line.split(" bags contain ");
            let key = tokens.next().unwrap();
            let values = tokens.next().unwrap();
            let v = if values == "no other bags." {
                vec![]
            } else {
                values
                    .trim_end_matches(".")
                    .split(", ")
                    .map(|value| {
                        let mut tokens = value.split_whitespace();
                        let n = tokens.next().unwrap().parse::<usize>().unwrap();
                        let spec = format!("{} {}", tokens.next().unwrap(), tokens.next().unwrap());
                        (spec, n)
                    })
                    .collect()
            };
            (key.to_string(), v)
        })
        .collect();
    let part_1 = specs
        .keys()
        .filter(|h| contains_at_least_one(&specs, h, "shiny gold"))
        .count();
    dbg!(part_1 - 1);
    dbg!(bags_inside(&specs, "shiny gold"));
}
