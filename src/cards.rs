extern crate rand;

use std::fmt;
use rand::thread_rng;
use rand::seq::SliceRandom;


#[derive(Debug)]
#[derive(Clone)]
pub struct Card {
    index: i16,
    string: String,
    suit: String,
}

impl Card {
    pub fn get_facevalue(&self) -> &String {
        &self.string
    }

    pub fn get_index(&self) -> i16 {
        self.index
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.string)
    }
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Deck {
    pub cards: Vec<Card>
}

impl Deck {
    pub fn shuffle(mut self) -> Deck {
        self.cards.shuffle(&mut thread_rng());
        self
    }

    pub fn take_one(mut self) -> Card {
        // probably need to return deck here also
        self.cards.pop().unwrap()
    }

    pub fn get_cards(&self) -> Vec<Card> {
        self.cards.clone()
    }

    pub fn add_cards(mut self, card: Card) -> Deck {
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

pub fn create_card(index:i16, string:String, suit:String) -> Card {
    Card{
        index: index,
        string: string,
        suit: suit
    }
}

pub fn create_deck(cards: Vec<Card>) -> Deck {
    Deck{
        cards: cards,
    }
}
