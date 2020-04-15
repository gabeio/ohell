#[path = "./cards.rs"] mod cards;

use std::collections::HashMap;
use std::cmp::{PartialEq, Eq};
use uuid::Uuid;

#[derive(Debug)]
#[derive(Clone)]
pub struct Ohhell {
    deck: cards::Deck,
    hands: Vec<Hand>,
    players: Vec<Player>,
    pile: cards::Deck,
}

impl Ohhell {
    pub fn new() -> Ohhell {
        Ohhell{
            deck: cards::Deck::new(vec!()),
            hands: vec!(),
            players: vec!(),
            pile: cards::Deck::new(vec!()),
        }
    }

    pub fn start(mut self) -> Ohhell {
        let deck = cards::Deck::new(vec!());
        let deck = deck.create_52();
        let deck = deck.shuffle();
        self.deck = deck;
        let ohhell = self.deal();
        self = ohhell;
        println!("{:?}", deck.get_cards());
        self.deck = deck;

    }

    // maybe change count to i16 later
    pub fn set_players(mut self, count: usize) -> Ohhell {
        for i in 0..count {
            let name: String = format!("Player {}", i);
            self.players.push(Player::new(name));
        }
        self
    }

    pub fn set_hands(mut self, count: usize) -> Ohhell {
        for _ in 0..count {
            let hand = Hand{
                winner: None
            };
            self.hands.push(hand);
        }
        self
    }

    fn deal(mut self) -> Ohhell {
        let mut players = self.players;
        for _ in 0..self.hands.len() {
            for player in &mut players {
                let deck = self.deck;
                println!("{}", deck.len());
                let (card, deck) = deck.take_card();
                let card = card.expect("there was no card to take");
                self.deck = deck;
                player.add_card(card);
            }
        }
        self.players = players;
        self
    }
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Round {
    hands: Vec<Hand>,
    bets: HashMap<Trick, bool>
}

impl Round {
    pub fn new() -> Round {
        Round{
            hands: vec!(),
            bets: HashMap::new(),
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

#[derive(Hash)]
#[derive(Debug)]
#[derive(Clone)]
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

#[derive(Debug)]
#[derive(Clone)]
pub struct Hand {
    winner: Option<Player>,
}

impl Hand {
    pub fn new() -> Hand {
        Hand{
            winner: None
        }
    }

    pub fn set_winner(mut self, player: Player) -> Hand {
        self.winner = Some(player);
        self
    }
}

#[derive(Hash)]
#[derive(Debug)]
#[derive(Clone)]
pub struct Player {
    uuid: Uuid,
    name: String,
    cards: Vec<cards::Card>,
}

impl Player {
    pub fn new(name: String) -> Player {
        Player{
            uuid: Uuid::new_v4(),
            name: name,
            cards: vec!(),
        }
    }

    pub fn add_card(&mut self, card: cards::Card) -> &mut Player {
        self.cards.push(card);
        self
    }

    pub fn take_card(mut self) -> (cards::Card, Player) {
        // probably need to return deck here also
        (self.cards.pop().unwrap(), self)
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

impl Eq for Player {}
