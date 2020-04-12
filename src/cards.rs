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

pub fn create_card(index:i16, string:String, suit:String) -> Card {
    Card{
        index: index,
        string: string,
        suit: suit
    }
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Deck {
    cards: Vec<Card>
}

impl Deck {
    pub fn create_52(mut self) -> Deck {
        let indexes = vec![1,2,3,4,5,6,7,8,9,10,11,12,13];
        let facevalues = vec!["A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K"];
        let suits:Vec<String> = vec!["hearts".to_owned(), "spades".to_owned(), "diamonds".to_owned(), "clubs".to_owned()];
        for suit in &suits {
            for i in 0..indexes.len() {
                let card = create_card(indexes[i], facevalues[i].to_owned(), suit.to_string());
                self = self.add_card(card);
            }
        }
        self
    }

    pub fn shuffle(mut self) -> Deck {
        self.cards.shuffle(&mut thread_rng());
        self
    }

    pub fn take_card(mut self) -> (Card, Deck) {
        // probably need to return deck here also
        (self.cards.pop().unwrap(), self)
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

pub fn create_deck(cards: Vec<Card>) -> Deck {
    Deck{
        cards: cards,
    }
}
