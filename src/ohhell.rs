#[path = "./cards.rs"] mod cards;

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

pub fn create_hand() -> Hand {
    Hand{
        winner: None
    }
}

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

pub fn create_player(name: String) -> Player {
    Player{
        name: name,
        cards: vec!(),
    }
}
