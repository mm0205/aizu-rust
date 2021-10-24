//! ITP1_6_Bの回答
//! [https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_6_B](https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_6_B)

use std::borrow::BorrowMut;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::io::BufRead;
use std::str::FromStr;

#[allow(dead_code)]
pub fn main() {
    loop {
        if let Some(cards) = read_input(std::io::stdin().lock()) {
            let missing_cards = compute_missing_cards(cards);
            for card in missing_cards {
                println!("{}", card);
            }
        }
        return;
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Suits {
    Spade,
    Heart,
    Club,
    Diamond,
}

impl Suits {
    fn all() -> Vec<Suits> {
        vec![Suits::Spade, Suits::Heart, Suits::Club, Suits::Diamond]
    }
}

impl FromStr for Suits {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "S" => Ok(Suits::Spade),
            "H" => Ok(Suits::Heart),
            "C" => Ok(Suits::Club),
            "D" => Ok(Suits::Diamond),
            _ => Err(())
        }
    }
}

impl Display for Suits {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Suits::Spade => "S",
            Suits::Heart => "H",
            Suits::Club => "C",
            Suits::Diamond => "D",
        };
        write!(f, "{}", text)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Card {
    suit: Suits,
    rank: i32,
}

impl Card {
    fn new(suit: Suits, rank: i32) -> Card {
        Card { suit, rank }
    }
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields: Vec<&str> = s.trim().split(' ').collect();
        if fields.len() != 2 {
            return Err(());
        }

        let suit = match Suits::from_str(fields[0]) {
            Ok(s) => s,
            _ => return Err(())
        };
        let rank = match fields[1].parse::<i32>() {
            Ok(n) => n,
            Err(_) => return Err(())
        };

        Ok(Card::new(suit, rank))
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.suit, self.rank)
    }
}


fn read_card<T: BufRead>(mut reader: T) -> Option<Card> {
    let mut line = String::new();
    if let Err(_) = reader.read_line(&mut line) {
        return None;
    }
    match Card::from_str(&line) {
        Ok(c) => Some(c),
        Err(_) => None
    }
}

fn read_input<T: BufRead>(mut reader: T) -> Option<HashSet<Card>> {
    let mut line = String::new();
    if let Err(_) = reader.read_line(&mut line) {
        return None;
    }

    let card_count = match line.trim().parse::<i32>() {
        Ok(n) => n,
        Err(_) => return None
    };

    let mut result = HashSet::new();
    for _ in 0..card_count {
        let card = match read_card(reader.borrow_mut()) {
            None => return None,
            Some(c) => c
        };
        result.insert(card);
    }

    Some(result)
}

fn create_all_cards() -> HashSet<Card> {
    let mut map = HashSet::new();
    for suit in Suits::all() {
        for rank in 1..=13 {
            let card = Card::new(suit, rank);
            map.insert(card);
        }
    }
    map
}

fn compute_missing_cards(existing_cards: HashSet<Card>) -> Vec<Card> {
    let mut all = create_all_cards();
    existing_cards.iter().for_each(|x| {
        all.remove(x);
    });

    let mut result = Vec::new();
    for suit in Suits::all() {
        for rank in 1..=13 {
            let card = Card::new(suit, rank);
            if all.contains(&card) {
                result.push(card);
            }
        }
    }
    result
}

#[cfg(test)]
mod test {
    use std::io::Cursor;
    use super::*;

    #[test]
    fn test_suits_from_str() {
        assert_eq!(Ok(Suits::Spade), Suits::from_str("S"));
        assert_eq!(Ok(Suits::Heart), Suits::from_str("H"));
        assert_eq!(Ok(Suits::Club), Suits::from_str("C"));
        assert_eq!(Ok(Suits::Diamond), Suits::from_str("D"));
    }

    #[test]
    fn test_card_from_str() {
        assert_eq!(Ok(Card { suit: Suits::Spade, rank: 1 }), Card::from_str("S 1"));
        assert_eq!(Ok(Card { suit: Suits::Heart, rank: 2 }), Card::from_str("H 2"));
        assert_eq!(Ok(Card { suit: Suits::Club, rank: 13 }), Card::from_str("C 13"));
        assert_eq!(Ok(Card { suit: Suits::Diamond, rank: 12 }), Card::from_str("D 12"));
    }

    #[test]
    fn test_read_card() {
        let input = Cursor::new(b"S 10");
        assert_eq!(Some(Card { suit: Suits::Spade, rank: 10 }), read_card(input));
        let input = Cursor::new(b"H 1");
        assert_eq!(Some(Card { suit: Suits::Heart, rank: 1 }), read_card(input));
        let input = Cursor::new(b"C 2");
        assert_eq!(Some(Card { suit: Suits::Club, rank: 2 }), read_card(input));
        let input = Cursor::new(b"D 7");
        assert_eq!(Some(Card { suit: Suits::Diamond, rank: 7 }), read_card(input));
    }

    #[test]
    fn test_read_input() {
        let input = Cursor::new(b"1\nS 10");
        let mut expected = HashSet::new();
        expected.insert(Card::new(Suits::Spade, 10));
        assert_eq!(Some(expected), read_input(input));

        let input = Cursor::new(b"2\nS 10\nD 13");
        let mut expected = HashSet::new();
        expected.insert(Card::new(Suits::Spade, 10));
        expected.insert(Card::new(Suits::Diamond, 13));
        assert_eq!(Some(expected),
                   read_input(input));
    }

    #[test]
    fn test_suits_all() {
        assert_eq!(vec![Suits::Spade, Suits::Heart, Suits::Club, Suits::Diamond], Suits::all());
    }

    #[test]
    fn test_create_all_card() {
        let mut cards = create_all_cards();
        for suit in Suits::all() {
            for rank in 1..=13 {
                let card = Card::new(suit, rank);
                assert!(cards.remove(&card));
            }
        }
        assert!(cards.is_empty());
    }

    #[test]
    fn test_compute_missing_cards() {
        let mut expected = Vec::new();
        expected.push(Card::new(Suits::Spade, 1));
        expected.push(Card::new(Suits::Diamond, 13));

        let mut input = create_all_cards();
        for x in expected.iter() {
            input.remove(x);
        }

        assert_eq!(expected, compute_missing_cards(input));
    }
}
