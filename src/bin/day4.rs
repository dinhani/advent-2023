use std::collections::HashMap;
use std::collections::HashSet;

use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::space1;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;

fn main() -> eyre::Result<()> {
    let input = include_str!("day4.txt");

    // parse cards
    let mut parser = tuple((
        parse_card_id,
        parse_numbers,
        space1,
        tag("|"),
        space1,
        parse_numbers,
    ));

    let mut cards = Vec::new();
    for line in input.lines() {
        let (_, (id, winners, _, _, _, current)) = parser(line)?;
        let card = Card {
            id,
            winners,
            current,
        };
        cards.push(card);
    }

    // calculate part 1
    let total_p1: usize = cards.iter().map(|c| c.points()).sum();
    println!("Part 1: {}", total_p1);

    // calculate part 2
    let mut card_copies: HashMap<usize, usize> = HashMap::with_capacity(cards.len());
    for card in &cards {
        card_copies.entry(card.id).or_insert(1);
    }
    for card in &cards {
        let card_copy_count = *card_copies.get(&card.id).unwrap();
        for card_copy in card.copies() {
            *card_copies.entry(card_copy).or_insert(0) += card_copy_count;
        }
    }

    let total_p2: usize = card_copies.values().sum();
    println!("Part 2: {}", total_p2);

    Ok(())
}

// -----------------------------------------------------------------------------
// NOM parsers
// -----------------------------------------------------------------------------
fn parse_card_id(input: &str) -> IResult<&str, usize> {
    let (remaining, (_, _, card_id, _, _)) =
        tuple((tag("Card"), space1, digit1, tag(":"), space1))(input)?;
    let game_id = card_id.parse::<usize>().unwrap();
    Ok((remaining, game_id))
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<usize>> {
    let (remaining, numbers) = separated_list1(space1, digit1)(input)?;
    let numbers: Vec<usize> = numbers.into_iter().map(|x| x.parse().unwrap()).collect();
    Ok((remaining, numbers))
}

// -----------------------------------------------------------------------------
// Structs
// -----------------------------------------------------------------------------
#[derive(Debug)]
struct Card {
    id: usize,
    winners: Vec<usize>,
    current: Vec<usize>,
}

impl Card {
    fn points(&self) -> usize {
        let matches = self.matches();
        if matches > 1 {
            usize::pow(2, (matches - 1) as u32)
        } else {
            matches
        }
    }

    fn copies(&self) -> Vec<usize> {
        let matches = self.matches();
        if matches > 0 {
            ((self.id + 1)..=(self.id + matches)).collect()
        } else {
            vec![]
        }
    }

    fn matches(&self) -> usize {
        let a: HashSet<&usize> = HashSet::from_iter(self.winners.iter());
        let b: HashSet<&usize> = HashSet::from_iter(self.current.iter());
        a.intersection(&b).collect::<HashSet<_>>().len()
    }
}
