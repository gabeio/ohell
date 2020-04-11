mod cards;

fn main() {
    let indexes = vec![1,2,3,4,5,6,7,8,9,10,11,12,13];
    let facevalues = vec!["A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K"];
    let suits:Vec<String> = vec!["hearts".to_owned(), "spades".to_owned(), "diamonds".to_owned(), "clubs".to_owned()];
    let deck:Vec<cards::Card> = Vec::new();
    let mut deck = cards::create_deck(deck);
    for suit in &suits {
        for i in 0..indexes.len() {
            let card = cards::create_card(indexes[i], facevalues[i].to_owned(), suit.to_string());
            deck = deck.add_cards(card);
        }
    }
    println!("{:?}", deck.get_cards());
    println!("{}", deck.len());
    deck = deck.shuffle();
    println!("{:?}", deck.get_cards());
}
