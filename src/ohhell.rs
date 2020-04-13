#[path = "./cards.rs"] mod cards;

#[derive(Debug)]
#[derive(Clone)]
pub struct Ohhell {
    deck: Option<cards::Deck>,
    hands: Vec<Hand>,
    players: Vec<Player>,
}

impl Ohhell {
    pub fn launch(mut self) {
        self.deck = Some(cards::create_deck(vec!()));
        self.deck = Some(self.deck.expect("expect A").create_52());
        self.deck = Some(self.deck.expect("expect B").shuffle());
        println!("{:?}", self.deck.expect("expect C").get_cards());
        // self.deck = Some(self.deck.expect(""))
    }

    fn deal(mut self) -> Ohhell {
        for player in &self.players {
            let (card, deck) = self.deck.expect("deal.deck.expect").take_card();
            self.deck = Some(deck);
            // player.add_card(card);
        }
        self
    }
}

pub fn create_ohhell() -> Ohhell {
    Ohhell{
        deck: None,
        hands: vec!(),
        players: vec!(),
    }
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Hand {
    winner: Option<Player>,
}

impl Hand {
    pub fn set_winner(mut self, player: Player) -> Hand {
        self.winner = Some(player);
        self
    }
}

pub fn create_hand() -> Hand {
    Hand{
        winner: None
    }
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Player {
    name: &'static str,
    cards: Vec<cards::Card>,
}

impl Player {
    pub fn add_card(mut self, card: cards::Card) -> Player {
        self.cards.push(card);
        self
    }

    pub fn take_card(mut self) -> (cards::Card, Player) {
        // probably need to return deck here also
        (self.cards.pop().unwrap(), self)
    }
}

pub fn create_player(name: &'static str) -> Player {
    Player{
        name: name,
        cards: vec!(),
    }
}
