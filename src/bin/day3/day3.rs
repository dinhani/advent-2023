use std::collections::HashSet;
use std::fmt::Debug;
use std::ops::RangeInclusive;

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
    let parser = alt((parse_number, parse_symbol, parse_empty, parse_line_ending));
    let engine_parts: Vec<EnginePart> = many0(parser)(input)?.1.into_iter().flatten().collect();

    // generate grid
    let grid_rows = engine_parts.iter().last().unwrap().line() + 1;
    let grid_cols = engine_parts.iter().last().unwrap().column_end() + 1;
    let mut grid: Vec<Vec<Option<&EnginePart>>> = vec![vec![None; grid_cols]; grid_rows];
    for part in &engine_parts {
        for part_col in part.column_range() {
            grid[part.line()][part_col] = Some(part)
        }
    }

    // extract engine elements
    let symbols: Vec<_> = engine_parts
        .iter()
        .filter_map(|x| match x {
            EnginePart::Symbol { gear, .. } => Some((x, gear)),
            _ => None,
        })
        .collect();

    let numbers: Vec<_> = engine_parts
        .iter()
        .filter_map(|x| match x {
            EnginePart::Number { number, .. } => Some((x, number)),
            _ => None,
        })
        .collect();

    // -------------------------------------------------------------------------
    // for each number, sum the ones connected to a symbol
    // -------------------------------------------------------------------------
    let mut numbers_total: usize = 0;
    for number in &numbers {
        for (row, col) in number.0.adjancent_positions() {
            if row >= grid_rows || col >= grid_cols {
                continue;
            }
            if let Some(EnginePart::Symbol { .. }) = grid[row][col] {
                numbers_total += number.1;
                break;
            }
        }
    }
    println!("Part 1: {}", numbers_total);

    //
    // -------------------------------------------------------------------------
    // for each gear, sum the ones connected to two numbers
    // -------------------------------------------------------------------------
    let mut gears_total: usize = 0;
    for symbol in &symbols {
        // check is gear
        if !symbol.1 {
            continue;
        }

        // extract connected numbers
        let mut connected = HashSet::new();
        for (row, col) in symbol.0.adjancent_positions() {
            if row >= grid_rows || col >= grid_cols {
                continue;
            }
            if let Some(number_part @ EnginePart::Number { number, .. }) = grid[row][col] {
                connected.insert((number_part, *number));
            }
        }

        // increment only if connected to two
        if connected.len() == 2 {
            gears_total += connected.iter().map(|x| x.1).product::<usize>()
        }
    }
    println!("Part 2: {}", gears_total);

    Ok(())
}

// -----------------------------------------------------------------------------
// NOM parsers
// -----------------------------------------------------------------------------
fn parse_number(input: Span) -> IResult<Span, Option<EnginePart>> {
    let (remaining, span) = digit1(input)?;
    Ok((
        remaining,
        Some(EnginePart::Number {
            span,
            number: span.parse::<usize>().unwrap(),
        }),
    ))
}

fn parse_symbol(input: Span) -> IResult<Span, Option<EnginePart>> {
    let (remaining, span) = is_not(".0123456789\n")(input)?;
    Ok((
        remaining,
        Some(EnginePart::Symbol {
            span,
            gear: span.fragment() == &"*",
        }),
    ))
}

fn parse_empty(input: Span) -> IResult<Span, Option<EnginePart>> {
    let (remaining, span) = tag(".")(input)?;
    Ok((remaining, Some(EnginePart::Empty { span })))
}

fn parse_line_ending(input: Span) -> IResult<Span, Option<EnginePart>> {
    let (remaining, _) = line_ending(input)?;
    Ok((remaining, None))
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
    fn adjancent_positions(&self) -> Vec<(usize, usize)> {
        let mut adjancent = Vec::new();
        // top left
        adjancent.push((
            self.line().saturating_sub(1),
            self.column_start().saturating_sub(1),
        ));

        // top right
        adjancent.push((self.line().saturating_sub(1), self.column_end() + 1));

        // top
        for col in self.column_range() {
            adjancent.push((self.line().saturating_sub(1), col));
        }

        // bottom left
        adjancent.push((self.line() + 1, self.column_start().saturating_sub(1)));

        // bottom right
        adjancent.push((self.line() + 1, self.column_end() + 1));

        // bottom
        for col in self.column_range() {
            adjancent.push((self.line() + 1, col));
        }

        // left
        adjancent.push((self.line(), self.column_start().saturating_sub(1)));

        // right
        adjancent.push((self.line(), self.column_end() + 1));

        adjancent
    }

    fn line(&self) -> usize {
        match self {
            EnginePart::Empty { span } => (span.location_line() as usize) - 1,
            EnginePart::Symbol { span, .. } => (span.location_line() as usize) - 1,
            EnginePart::Number { span, .. } => (span.location_line() as usize) - 1,
        }
    }

    fn column_start(&self) -> usize {
        match self {
            EnginePart::Empty { span } => span.get_column() - 1,
            EnginePart::Symbol { span, .. } => span.get_column() - 1,
            EnginePart::Number { span, .. } => span.get_column() - 1,
        }
    }

    fn column_end(&self) -> usize {
        match self {
            EnginePart::Empty { .. } => self.column_start(),
            EnginePart::Symbol { .. } => self.column_start(),
            EnginePart::Number { span, .. } => self.column_start() + span.fragment().len() - 1,
        }
    }

    fn column_range(&self) -> RangeInclusive<usize> {
        self.column_start()..=self.column_end()
    }
}
