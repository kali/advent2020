use nom::branch::alt;
use nom::character::complete::{char as cchar, digit1, multispace0};
use nom::combinator::{map, map_res};
use nom::multi::{fold_many0, separated_list1};
use nom::sequence::{delimited, pair};
use nom::IResult;

fn main() {
    let lines = std::fs::read_to_string("input").unwrap();
    dbg!(lines.lines().map(|l| expr1(l).unwrap().1).sum::<isize>());
    dbg!(lines.lines().map(|l| expr2(l).unwrap().1).sum::<isize>());
}

fn expr1(input: &str) -> IResult<&str, isize> {
    let (i, init) = atom(input, expr1)?;
    fold_many0(
        pair(alt((cchar('+'), cchar('*'))), |i| atom(i, expr1)),
        init,
        |acc, (op, v)| if op == '*' { acc * v } else { acc + v },
    )(i)
}

fn expr2(input: &str) -> IResult<&str, isize> {
    fn sum(input: &str) -> IResult<&str, isize> {
        map(separated_list1(cchar('+'), |i| atom(i, expr2)), |vs| {
            vs.iter().sum()
        })(input)
    }

    map(separated_list1(cchar('*'), sum), |vs| vs.iter().product())(input)
}

fn atom<E>(input: &str, e: E) -> IResult<&str, isize>
where
    E: for<'a> FnMut(&'a str) -> IResult<&'a str, isize>,
{
    alt((
        delimited(ws(cchar('(')), e, ws(cchar(')'))),
        ws(map_res(digit1, |n: &str| n.parse())),
    ))(input)
}

fn ws<'a, F: 'a, O>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    F: FnMut(&'a str) -> IResult<&'a str, O>,
{
    delimited(multispace0, inner, multispace0)
}
