use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let foods: Vec<(Vec<&str>, HashSet<&str>)> = input
        .lines()
        .map(|l| {
            let mut food = l.trim_end_matches(")").split(" (contains ");
            (
                food.next().unwrap().split_whitespace().collect(),
                food.next().unwrap().split(", ").collect(),
            )
        })
        .collect();
    let allergens = foods
        .iter()
        .flat_map(|f| f.1.iter().copied())
        .sorted()
        .unique()
        .collect::<Vec<&str>>();
    let mut identified = HashMap::new();
    while allergens.len() > identified.len() {
        for allergen in &allergens {
            if identified.contains_key(allergen) {
                continue;
            }
            let candidates = foods
                .iter()
                .filter(|f| f.1.contains(allergen))
                .map(|f| {
                    f.0.iter()
                        .filter(|i| !identified.values().any(|i2| i == &i2))
                        .copied()
                        .collect::<HashSet<&str>>()
                })
                .fold1(|a, b| a.intersection(&b).copied().collect())
                .unwrap();
            if candidates.len() == 1 {
                identified.insert(allergen, candidates.iter().next().copied().unwrap());
            }
        }
    }
    let p1 = foods
        .iter()
        .flat_map(|f| f.0.iter())
        .filter(|i| !identified.values().any(|i2| i == &i2))
        .count();
    dbg!(p1);
    let p2 = identified.iter().sorted_by_key(|p| p.0).map(|p| p.1).join(",");
    dbg!(p2);
}
