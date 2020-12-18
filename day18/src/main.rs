use nom::branch::alt;
use nom::character::complete::char as cchar;
use nom::character::complete::digit1;
use nom::character::complete::multispace0;
use nom::combinator::map;
use nom::combinator::map_res;
use nom::error::ParseError;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::sequence::pair;
use nom::IResult;

fn main() {
    let lines = std::fs::read_to_string("input").unwrap();
    let p1 = lines.lines().map(|l| expr1(l).unwrap().1).sum::<isize>();
    dbg!(p1);
    let p2 = lines.lines().map(|l| expr2(l).unwrap().1).sum::<isize>();
    dbg!(p2);
}

fn token<E>(input: &str, e: E) -> IResult<&str, isize>
where
    E: for<'a> FnMut(&'a str) -> IResult<&'a str, isize>,
{
    alt((
        delimited(ws(cchar('(')), e, ws(cchar(')'))),
        ws(map_res(digit1, |n: &str| n.parse())),
    ))(input)
}

fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

fn expr1(input: &str) -> IResult<&str, isize> {
    let (i, init) = token(input, expr1)?;
    nom::multi::fold_many0(
        pair(ws(alt((cchar('+'), cchar('*')))), |i| token(i, expr1)),
        init,
        |acc, (op, v)| match op {
            '+' => acc + v,
            '*' => acc * v,
            _ => panic!(),
        },
    )(i)
}

fn expr2(input: &str) -> IResult<&str, isize> {
    map(separated_list1(cchar('*'), sum), |vs| vs.iter().product())(input)
}

fn sum(input: &str) -> IResult<&str, isize> {
    map(separated_list1(cchar('+'), |i| token(i, expr2)), |vs| {
        vs.iter().sum()
    })(input)
}
