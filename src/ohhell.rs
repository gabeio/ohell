use std::collections::HashMap;
use std::cmp::{PartialEq, Eq};

use uuid::Uuid;

use crate::cards::{Card, Deck, Suit};

#[derive(Debug, Clone)]
pub struct Ohhell {
    rounds: Vec<Round>,
    players: Vec<Player>,
}

impl Ohhell {
    pub fn new() -> Ohhell {
        Ohhell{
            rounds: vec!(),
            players: vec!(),
        }
    }

    pub fn start(self) -> Ohhell {
        // let deck = cards::Deck::new(vec!());
        // let deck = deck.create_52();
        // let deck = deck.shuffle();
        // self.deck = deck;
        // let ohhell = self.deal();
        // self = ohhell;
        self
    }

    // maybe change count to i16 later
    pub fn set_players(mut self, count: usize) -> Ohhell {
        for i in 0..count {
            let name: String = format!("Player {}", i);
            self.players.push(Player::new(name));
        }
        self
    }

    pub fn set_rounds(mut self, count: usize) -> Ohhell {
        for _ in 0..count {
            self.rounds.push(Round::new());
        }
        self
    }

    // fn deal(mut self) -> Ohhell {
    //     let mut players = self.players;
    //     for _ in 0..self.rounds.len() {
    //         for player in &mut players {
    //             let deck = self.deck;
    //             println!("{}", deck.len());
    //             let (card, deck) = deck.take_card();
    //             let card = card.expect("there was no card to take");
    //             self.deck = deck;
    //             player.add_card(card);
    //         }
    //     }
    //     self.players = players;
    //     self
    // }
}

#[derive(Debug, Clone)]
pub struct Round {
    hands: Vec<Hand>,
    bets: HashMap<Trick, bool>,
    deck: Deck,
}

impl Round {
    pub fn new() -> Round {
        Round{
            hands: vec!(),
            bets: HashMap::new(),
            deck: Deck::new(vec!()),
        }
    }

    pub fn add_hands(mut self, hands: usize) -> Round {
        for _ in 0..hands {
            self.hands.push(Hand::new());
        }
        self
    }

    pub fn set_winners(mut self, winners: Vec<Player>) -> Round {
        let mut bets = self.bets;
        for winner in winners.iter() {
            for (trick, win) in bets.iter_mut() {
                if trick.player == *winner {
                    *win = true;
                }
            }
        }
        self.bets = bets;
        self
    }
}

#[derive(Hash, Debug, Clone)]
pub struct Trick {
    uuid: Uuid,
    player: Player,
    tricks: i16,
}

impl Trick {
    pub fn new(player: Player, bet: i16) -> Trick {
        Trick{
            uuid: Uuid::new_v4(),
            player: player,
            tricks: bet,
        }
    }
}

impl PartialEq for Trick {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

impl Eq for Trick {}

#[derive(Debug, Clone)]
pub struct Hand {
    winner: Option<Player>,
    pile: Deck,
}

impl Hand {
    pub fn new() -> Hand {
        Hand{
            winner: None,
            pile: Deck::new(vec!()),
        }
    }

    pub fn add_card(mut self, card: Card) -> Hand {
        self.pile = self.pile.add_card(card);
        self
    }

    pub fn set_winner(mut self, player: Player) -> Hand {
        self.winner = Some(player);
        self
    }
}

#[derive(Hash, Debug, Clone)]
pub struct Player {
    uuid: Uuid,
    name: String,
    cards: Deck,
}

impl Player {
    pub fn new(name: String) -> Player {
        Player{
            uuid: Uuid::new_v4(),
            name: name,
            cards: Deck::new(vec!()),
        }
    }

    pub fn add_card(mut self, card: Card) -> Player {
        self.cards = self.cards.add_card(card);
        self
    }

    pub fn take_a_card(mut self) -> (Card, Player) {
        let (card, deck) = self.cards.take_a_card();
        self.cards = deck;
        (card, self)
    }

    pub fn take_card(mut self, index: i8, suit: i8) -> (Card, Player) {
        let (card, deck) = self.cards.take_card(index, suit);
        self.cards = deck;
        (card, self)
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

impl Eq for Player {}
