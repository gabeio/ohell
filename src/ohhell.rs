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
            self.players.push(create_player(name));
        }
        self
    }

    pub fn set_hands(mut self, count: usize) -> Ohhell {
        for i in 0..count {
            let hand = Hand{
                winner: None
            };
            self.hands.push(hand);
        }
        self
    }

    fn deal(mut self) -> Ohhell {
        let mut players = self.players;
        for player in &mut players {
            let deck = self.deck;
            let (card, deck) = deck.take_card();
            self.deck = deck;
            player.add_card(card);
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
    name: String,
    cards: Vec<cards::Card>,
}

impl Player {
    pub fn add_card(&mut self, card: cards::Card) -> &mut Player {
        self.cards.push(card);
        self
    }

    pub fn take_card(mut self) -> (cards::Card, Player) {
        // probably need to return deck here also
        (self.cards.pop().unwrap(), self)
    }
}

pub fn create_player(name: String) -> Player {
    Player{
        name: name,
        cards: vec!(),
    }
}
