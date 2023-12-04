use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::sequence::tuple;
use nom::IResult;

fn main() -> eyre::Result<()> {
    let input = include_str!("day2.txt");

    // parse games
    let mut parser = tuple((parse_game_id, parse_sets));

    let mut games = Vec::new();
    for line in input.lines() {
        let (_, (id, sets)) = parser(line)?;
        let game = Game { id, sets };
        games.push(game);
    }

    // calculate part 1 answer
    let valid_games_ids: Vec<usize> = games
        .iter()
        .filter(|g| {
            g.max(Color::Red) <= 12 && g.max(Color::Green) <= 13 && g.max(Color::Blue) <= 14
        })
        .map(|g| g.id)
        .collect();
    let total_p1: usize = valid_games_ids.into_iter().sum();
    println!("Part 1: {}", total_p1);

    // calculate part 2 answer
    let game_draw_products: Vec<usize> = games
        .iter()
        .map(|g| {
            [g.max(Color::Red), g.max(Color::Green), g.max(Color::Blue)]
                .iter()
                .product()
        })
        .collect();
    let total_p2: usize = game_draw_products.iter().sum();
    println!("Part 2: {}", total_p2);

    Ok(())
}

// -----------------------------------------------------------------------------
// NOM parsers
// -----------------------------------------------------------------------------
fn parse_game_id(input: &str) -> IResult<&str, usize> {
    let (remaining, (_, game_id, _)) = tuple((tag("Game "), digit1, tag(": ")))(input)?;
    let game_id = game_id.parse::<usize>().unwrap();
    Ok((remaining, game_id))
}

fn parse_sets(input: &str) -> IResult<&str, Vec<Set>> {
    separated_list1(tag("; "), parse_set)(input)
}

fn parse_set(input: &str) -> IResult<&str, Set> {
    let (remaining, draws) = separated_list1(tag(", "), parse_draw)(input)?;
    Ok((remaining, Set { draws }))
}

fn parse_draw(input: &str) -> IResult<&str, Draw> {
    let (remaining, (quantity, color)) = separated_pair(digit1, tag(" "), parse_color)(input)?;
    let draw = Draw {
        quantity: quantity.parse::<usize>().unwrap(),
        color: Color::try_from(color).unwrap(),
    };
    Ok((remaining, draw))
}

fn parse_color(input: &str) -> IResult<&str, &str> {
    alt((tag("red"), tag("blue"), tag("green")))(input)
}

// -----------------------------------------------------------------------------
// Structs
// -----------------------------------------------------------------------------
#[derive(Debug)]
struct Game {
    id: usize,
    sets: Vec<Set>,
}

impl Game {
    fn max(&self, color: Color) -> usize {
        self.sets
            .iter()
            .map(|set| set.max(color))
            .max()
            .unwrap_or(0)
    }
}

#[derive(Debug)]
struct Set {
    draws: Vec<Draw>,
}

impl Set {
    fn max(&self, color: Color) -> usize {
        self.draws
            .iter()
            .filter(|d| d.color == color)
            .map(|d| d.quantity)
            .max()
            .unwrap_or(0)
    }
}

#[derive(Debug)]
struct Draw {
    quantity: usize,
    color: Color,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, strum::EnumString)]
enum Color {
    #[strum(serialize = "red")]
    Red,
    #[strum(serialize = "green")]
    Green,
    #[strum(serialize = "blue")]
    Blue,
}
