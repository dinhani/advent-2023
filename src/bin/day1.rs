use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take;
use nom::character::complete::one_of;
use nom::combinator::value;
use nom::multi::many0;
use nom::IResult;

fn main() -> eyre::Result<()> {
    let input = include_str!("day1.txt");
    let mut parser_p1 = alt((parse_digit, parse_not_digit));
    let mut parser_p2 = alt((parse_digit, parse_digit_name, parse_not_digit));

    let mut total_p1: usize = 0;
    let mut total_p2: usize = 0;

    for line in input.lines() {
        // extract digits from line
        let (_, digits_p1) = many0(&mut parser_p1)(line)?;
        let (_, digits_p2) = many0(&mut parser_p2)(line)?;

        // remove non-digits
        let digits_p1: Vec<_> = digits_p1.into_iter().flatten().collect();
        let digits_p2: Vec<_> = digits_p2.into_iter().flatten().collect();

        // accumulate
        total_p1 +=
            ((digits_p1.first().unwrap_or(&0) * 10) + digits_p1.last().unwrap_or(&0)) as usize;
        total_p2 +=
            ((digits_p2.first().unwrap_or(&0) * 10) + digits_p2.last().unwrap_or(&0)) as usize;
    }

    println!("Part 1: {}", total_p1);
    println!("Part 2: {}", total_p2);
    Ok(())
}

// -----------------------------------------------------------------------------
// NOM parsers
// -----------------------------------------------------------------------------
fn parse_digit(input: &str) -> IResult<&str, Option<u8>> {
    let (remaining, value) = one_of("123456789")(input)?;
    Ok((remaining, Some(value.to_digit(10).unwrap() as u8)))
}

fn parse_digit_name(input: &str) -> IResult<&str, Option<u8>> {
    let (_, value) = alt((
        value(1, tag("one")),
        value(2, tag("two")),
        value(3, tag("three")),
        value(4, tag("four")),
        value(5, tag("five")),
        value(6, tag("six")),
        value(7, tag("seven")),
        value(8, tag("eight")),
        value(9, tag("nine")),
    ))(input)?;
    Ok((&input[1..input.len()], Some(value)))
}

fn parse_not_digit(input: &str) -> IResult<&str, Option<u8>> {
    let (remaining, _) = take(1u8)(input)?;
    Ok((remaining, None))
}
