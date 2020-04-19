use uuid::Uuid;

use crate::ohhell::Ohhell;

#[derive(Debug, Clone)]
pub struct Room {
    uuid: Uuid,
    pub game: Ohhell,
}

impl Room {
    //
    pub fn new() -> Room {
        Room {
            uuid: Uuid::new_v4(),
            game: Ohhell::new(),
        }
    }
}
