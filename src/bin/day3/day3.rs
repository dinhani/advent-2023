use std::collections::HashMap;
use std::fmt::Debug;

use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::multi::many0;
use nom::IResult;
use nom_locate::LocatedSpan;

type Span<'a> = LocatedSpan<&'a str>;

fn main() -> eyre::Result<()> {
    let input = Span::new(include_str!("day3.txt"));

    // parse engine parts
    let parser = alt((parse_number, parse_symbol, parse_empty));
    let (_, engine_parts) = many0(parser)(input)?;
    let symbols: Vec<_> = engine_parts
        .iter()
        .filter(|x| matches!(x, EnginePart::Symbol { .. }))
        .collect();
    let numbers: Vec<_> = engine_parts
        .iter()
        .filter(|x| matches!(x, EnginePart::Number { .. }))
        .collect();

    // calculate distance between each number position and each symbol to determine
    // numbers that are connected to symbols
    let mut connected_gears: HashMap<&EnginePart, Vec<usize>> = HashMap::new();
    let mut connected_numbers: Vec<usize> = Vec::with_capacity(numbers.len());
    for number in numbers {
        let mut added_to_connected = false;
        let mut added_to_gears = false;

        let number_value = match number {
            EnginePart::Number { number, .. } => number,
            _ => panic!("number should be EnginePart::Number"),
        };

        for number_col in number.column_start()..=number.column_end() {
            for symbol in &symbols {
                let symbol_is_gear = match symbol {
                    EnginePart::Symbol { gear, .. } => gear,
                    _ => panic!("symbol should be EnginePart::Symbol"),
                };

                // calculate distance between number and symbol
                let line_dist = (number.line() as isize - symbol.line() as isize).abs();
                let col_dist = (number_col as isize - symbol.column_start() as isize).abs();
                let connected = line_dist <= 1 && col_dist <= 1;

                // track number as connected
                if connected && !added_to_connected {
                    connected_numbers.push(*number_value);
                    added_to_connected = true;
                }

                // if gear, add number to gear map
                if connected && *symbol_is_gear && !added_to_gears {
                    connected_gears
                        .entry(symbol)
                        .or_default()
                        .push(*number_value);
                    added_to_gears = true;
                }
            }
        }
    }

    // calculate part 1
    let connected_numbers_sum: usize = connected_numbers.iter().sum();
    println!("Part 1: {}", connected_numbers_sum);

    // calculate part 2
    let connected_gears: Vec<usize> = connected_gears
        .values()
        .filter(|x| x.len() == 2)
        .map(|x| x.iter().product())
        .collect();

    let connected_gears_sum: usize = connected_gears.iter().sum();
    println!("Part 2: {}", connected_gears_sum);

    Ok(())
}

// -----------------------------------------------------------------------------
// NOM parsers
// -----------------------------------------------------------------------------
fn parse_number(input: Span) -> IResult<Span, EnginePart> {
    let (remaining, number) = digit1(input)?;
    Ok((
        remaining,
        EnginePart::Number {
            span: number,
            number: number.parse::<usize>().unwrap(),
        },
    ))
}

fn parse_symbol(input: Span) -> IResult<Span, EnginePart> {
    let (remaining, symbol) = is_not(".0123456789\n")(input)?;
    Ok((
        remaining,
        EnginePart::Symbol {
            span: symbol,
            gear: symbol.fragment() == &"*",
        },
    ))
}

fn parse_empty(input: Span) -> IResult<Span, EnginePart> {
    let (remaining, empty) = alt((tag("."), line_ending))(input)?;
    Ok((remaining, EnginePart::Empty { span: empty }))
}

// -----------------------------------------------------------------------------
// Structs
// -----------------------------------------------------------------------------
#[derive(Hash, Eq, PartialEq)]
enum EnginePart<'a> {
    Empty { span: Span<'a> },
    Symbol { span: Span<'a>, gear: bool },
    Number { span: Span<'a>, number: usize },
}

impl Debug for EnginePart<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty { .. } => write!(f, "Empty"),
            Self::Symbol { span, .. } => f
                .debug_tuple("Symbol")
                .field(&span.fragment())
                .field(&self.line())
                .field(&self.column_start())
                .finish(),
            Self::Number { span, .. } => f
                .debug_tuple("Number")
                .field(&span.fragment())
                .field(&self.line())
                .field(&self.column_start())
                .field(&self.column_end())
                .finish(),
        }
    }
}

impl EnginePart<'_> {
    fn line(&self) -> usize {
        match self {
            EnginePart::Empty { span } => span.location_line() as usize,
            EnginePart::Symbol { span, .. } => span.location_line() as usize,
            EnginePart::Number { span, .. } => span.location_line() as usize,
        }
    }

    fn column_start(&self) -> usize {
        match self {
            EnginePart::Empty { span } => span.get_column(),
            EnginePart::Symbol { span, .. } => span.get_column(),
            EnginePart::Number { span, .. } => span.get_column(),
        }
    }

    fn column_end(&self) -> usize {
        match self {
            EnginePart::Empty { span } => span.get_column(),
            EnginePart::Symbol { span, .. } => span.get_column(),
            EnginePart::Number { span, .. } => span.get_column() + span.fragment().len() - 1,
        }
    }
}
