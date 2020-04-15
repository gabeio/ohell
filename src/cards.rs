extern crate rand;

use std::fmt;
use rand::thread_rng;
use rand::seq::SliceRandom;

#[derive(Hash, Debug, Clone, Copy)]
pub struct Card {
    index: i8,
    facevalue: &'static str,
    suit: i8,
}

impl Card {
    pub fn new(index:i8, string:&'static str, suit: i8) -> Card {
        Card{
            index: index,
            facevalue: string,
            suit: suit
        }
    }

    pub fn get_facevalue(&self) -> &str {
        &self.facevalue
    }

    pub fn get_suit(&self) -> String {
        match self.suit {
            1 => "hearts".to_owned(),
            2 => "diamonds".to_owned(),
            3 => "clubs".to_owned(),
            4 => "spades".to_owned(),
            _ => panic!("card has invalid suit"),
        }
    }

    pub fn get_index(&self) -> i8 {
        self.index
    }

    pub fn get_suit_int(&self) -> i8 {
        self.suit
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.facevalue)
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.suit == other.suit
    }
}

impl Eq for Card {}

#[derive(Hash, Debug, Clone)]
pub struct Deck {
    cards: Vec<Card>
}

impl Deck {
    pub fn new(cards: Vec<Card>) -> Deck {
        Deck{
            cards: cards,
        }
    }

    pub fn create_52(mut self) -> Deck {
        let indexes = vec![1,2,3,4,5,6,7,8,9,10,11,12,13];
        let facevalues = vec!["A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K"];
        let suits = vec![1,2,3,4];
        for suit in &suits {
            for i in 0..indexes.len() {
                let card = Card::new(indexes[i], facevalues[i], *suit);
                self = self.add_card(card);
            }
        }
        self
    }

    pub fn shuffle(mut self) -> Deck {
        self.cards.shuffle(&mut thread_rng());
        self
    }

    pub fn take_a_card(mut self) -> (Card, Deck) {
        (self.cards.pop().unwrap(), self)
    }

    pub fn take_card(mut self, index: i8, suit: i8) -> (Card, Deck) {
        let card = Card::new(index, "", suit);
        // apparently remove_item is unstable
        // let card = self.cards.remove_item(&card).expect("you don't have that card");
        let pos = self.cards.iter().position(|x| *x == card).unwrap();
        let card = Some(self.cards.remove(pos));
        let card = card.expect("you don't have that card");
        (card, self)
    }

    pub fn get_cards(&self) -> Vec<Card> {
        self.cards.clone()
    }

    pub fn add_card(mut self, card: Card) -> Deck {
        self.cards.push(card);
        self
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for card in &self.cards {
            write!(f, "{}", card.get_facevalue())?;
        }
        write!(f,"")
    }
}
