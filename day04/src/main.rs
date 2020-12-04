use anyhow::*;
use std::collections::HashMap;

fn valid(rec: &HashMap<String, String>) -> Result<()> {
    let byr: usize = rec["byr"].parse().unwrap();
    let iyr: usize = rec["iyr"].parse().unwrap();
    let eyr: usize = rec["eyr"].parse().unwrap();
    if byr < 1920 || byr > 2002 || iyr < 2010 || iyr > 2020 || eyr < 2020 || eyr > 2030 {
        bail!("date");
    }
    let hgt = &rec["hgt"];
    match hgt.len() {
        4 if hgt.ends_with("in") => {
            let inches = hgt.trim_end_matches("in").parse::<usize>().unwrap();
            if inches < 59 || inches > 76 {
                bail!("inches");
            }
        }
        5 if hgt.ends_with("cm") => {
            let cm = hgt.trim_end_matches("cm").parse::<usize>().unwrap();
            if cm < 150 || cm > 193 {
                bail!("cms");
            }
        }
        _ => bail!("hgt"),
    }
    let hcl = &rec["hcl"];
    if hcl.len() != 7
        || hcl.as_bytes()[0] != b'#'
        || !hcl
            .bytes()
            .skip(1)
            .all(|b| b"0123456789abcdef".contains(&b))
    {
        bail!("hcl");
    }
    let ecl = &rec["ecl"];
    if "amb blu brn gry grn hzl oth"
        .split_whitespace()
        .find(|c| c == ecl)
        .is_none()
    {
        bail!("ecl");
    }
    let pid = &rec["pid"];
    if pid.len() != 9 || !pid.bytes().all(|b| b"0123456789".contains(&b)) {
        bail!("pid");
    }
    Ok(())
}

fn parse(input: &str) -> Vec<HashMap<String, String>> {
    input
        .split("\n\n")
        .map(|record| {
            record
                .split_whitespace()
                .map(|field| {
                    let mut tokens = field.split(":");
                    (
                        tokens.next().unwrap().to_owned(),
                        tokens.next().unwrap().to_owned(),
                    )
                })
                .collect()
        })
        .collect()
}

fn full(rec: &HashMap<String, String>) -> bool {
    "byr iyr eyr hgt hcl ecl pid"
        .split_whitespace()
        .all(|f| rec.contains_key(f))
}

fn main() {
    let records = parse(&std::fs::read_to_string("input").unwrap());
    dbg!(records.iter().filter(|h| full(h)).count());
    dbg!(records
        .iter()
        .filter(|h| full(h))
        .filter(|h| { valid(h).is_ok() })
        .count());
}

#[test]
fn test_invalid() {
    let recs = r#"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007"#;
    assert!(parse(recs)
        .iter()
        .all(|rec| !full(rec) || valid(rec).is_err()));
}

#[test]
fn test_valid() {
    let recs = r#"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
"#;
    assert!(parse(recs)
        .iter()
        .all(|rec| full(rec) && valid(rec).is_ok()));
}
