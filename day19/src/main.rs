use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::*;
use nom::combinator::*;
use nom::multi::*;
use nom::sequence::*;
use nom::IResult;
use std::collections::HashMap;

fn main() {
    let text = std::fs::read_to_string("input").unwrap();
    let (parser, msgs) = all_consuming(input)(&*text).unwrap().1;
    let p1 = msgs.iter().filter(|m| parser.valid(m)).count();
    dbg!(p1);
    let mut parser2 = parser.clone();
    parser2.0.insert(8, Rule::Alt(vec![vec![42], vec![42, 8]]));
    parser2
        .0
        .insert(11, Rule::Alt(vec![vec![42, 31], vec![42, 11, 31]]));
    let p2 = msgs.iter().filter(|m| parser2.valid(m)).count();
    dbg!(p2);
}

#[derive(Clone, Debug)]
struct Parser(HashMap<usize, Rule>);

#[derive(Clone, Debug)]
enum Rule {
    Alt(Vec<Vec<usize>>),
    Char(char),
}

impl Parser {
    fn valid(&self, input: &str) -> bool {
        self.parse(0, input.as_bytes()).iter().any(|l| l.len() == 0)
    }
    fn parse<'a>(&self, rule: usize, input: &'a [u8]) -> Vec<&'a [u8]> {
        let options = match &self.0[&rule] {
            Rule::Char(c) => {
                if input.len() > 0 && input[0] == *c as u8 {
                    vec![&input[1..]]
                } else {
                    vec![]
                }
            }
            Rule::Alt(alts) => alts
                .iter()
                .flat_map(|seq| {
                    seq.iter().fold(vec![input], |inputs, rule| {
                        inputs.iter().flat_map(|i| self.parse(*rule, i)).collect()
                    })
                })
                .collect(),
        };
        options
    }
}

fn input(i: &str) -> IResult<&str, (Parser, Vec<&str>)> {
    separated_pair(parser, tag("\n\n"), many1(terminated(alpha1, newline)))(i)
}

fn parser(i: &str) -> IResult<&str, Parser> {
    map(
        separated_list1(newline, separated_pair(number, tag(": "), rule)),
        |v| Parser(v.into_iter().collect()),
    )(i)
}

fn rule(i: &str) -> IResult<&str, Rule> {
    alt((
        map(separated_list1(tag(" | "), seq), Rule::Alt),
        map(delimited(tag("\""), anychar, tag("\"")), Rule::Char),
    ))(i)
}

fn seq(i: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(tag(" "), number)(i)
}

fn number(i: &str) -> IResult<&str, usize> {
    map_res(digit1, std::str::FromStr::from_str)(i)
}

#[test]
fn t0() {
    let parser = parser(
        r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b""#,
    )
    .unwrap()
    .1;
    assert!(parser.valid("ababbb"));
    assert!(parser.valid("abbbab"));
    assert!(!parser.valid("bababa"));
    assert!(!parser.valid("aaabbb"));
    assert!(!parser.valid("aaaabbb"));
}
