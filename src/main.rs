use cursive::Cursive;
use cursive::views::{TextView};

mod ohhell;

fn main() {
    let ohhell: ohhell::Ohhell = ohhell::create_ohhell();
    let ohhell = ohhell.set_players(4usize);
    let ohhell = ohhell.set_hands(7usize);
    ohhell.launch();

    let mut siv = Cursive::default();

    // We can quit by pressing `q`
    siv.add_global_callback('q', Cursive::quit);

    // Add a simple view
    siv.add_layer(TextView::new(
        "Hello World!\n\
        ♦\n\
        ♣\n\
        ♠\n\
        ♥\n\
         Press q to quit the application.",
    ));

    // Run the event loop
    siv.run();
}
