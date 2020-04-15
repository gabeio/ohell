#[path = "./ohhell.rs"] mod ohhell;

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Room {
    uuid: Uuid,
    game: ohhell::Ohhell,
}

impl Room {
    //
    pub fn new() -> Room {
        Room {
            uuid: Uuid::new_v4(),
            game: ohhell::Ohhell::new(),
        }
    }

    pub fn start(mut self) -> Room {
        self.game = self.game.start();
        self
    }
}