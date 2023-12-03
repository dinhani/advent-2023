use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::sequence::separated_pair;
use nom::IResult;

fn main() -> eyre::Result<()> {
    let input = include_str!("day2.txt");

    // parse games
    let mut games = Vec::with_capacity(100);
    for line in input.lines() {
        let (line, game_id) = parse_game_id(line)?;
        let (_, sets) = parse_sets(line)?;
        let game = Game { id: game_id, sets };
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
    let valid_game_ids_sum: usize = valid_games_ids.into_iter().sum();

    // calculate part 2 answer
    let game_draw_products: Vec<usize> = games
        .iter()
        .map(|g| {
            [g.max(Color::Red), g.max(Color::Green), g.max(Color::Blue)]
                .iter()
                .product()
        })
        .collect();
    let game_draw_products_sum: usize = game_draw_products.iter().sum();

    println!("Part 1: {}", valid_game_ids_sum);
    println!("Part 2: {}", game_draw_products_sum);
    Ok(())
}

// -----------------------------------------------------------------------------
// NOM parsers
// -----------------------------------------------------------------------------
fn parse_game_id(input: &str) -> IResult<&str, GameId> {
    let (remaining, game_id) = preceded(tag("Game "), digit1)(input)?;
    let game_id = game_id.parse::<usize>().unwrap();
    let (remaining, _) = tag(": ")(remaining)?;
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
    let (remaining, (qty, color)) = separated_pair(digit1, tag(" "), color)(input)?;
    let draw = Draw {
        quantity: qty.parse::<usize>().unwrap(),
        color: Color::try_from(color).unwrap(),
    };
    Ok((remaining, draw))
}

fn color(input: &str) -> IResult<&str, &str> {
    alt((tag("red"), tag("blue"), tag("green")))(input)
}

// -----------------------------------------------------------------------------
// Structs
// -----------------------------------------------------------------------------
type GameId = usize;

#[derive(Debug)]
struct Game {
    id: GameId,
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
