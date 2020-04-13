#[path = "./cards.rs"] mod cards;

#[derive(Debug)]
#[derive(Clone)]
pub struct Ohhell {
    deck: cards::Deck,
    hands: Vec<Hand>,
    players: Vec<Player>,
}

impl Ohhell {
    pub fn launch(mut self) {
        let deck = cards::create_deck(vec!());
        let deck = deck.create_52();
        let deck = deck.shuffle();
        let ohhell = self.deal();
        self = ohhell;
        println!("{:?}", deck.get_cards());
        self.deck = deck;

    }

    fn deal(mut self) -> Ohhell {
        let mut players = self.players;
        for player in &mut players {
            let deck = self.deck;
            let (card, deck) = deck.take_card();
            self.deck = deck;
            let mut _p = player;
            _p = &mut _p.add_card(card);
        }
        self.players = players;
        self
    }
}

pub fn create_ohhell() -> Ohhell {
    Ohhell{
        deck: cards::create_deck(vec!()),
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
