use maud::{html, Markup};

use super::game::Game;
use crate::games::game_id::GameId;

pub struct Dames {
    id: String,
    name: String,
}

impl Dames {
    pub fn new(game_id: GameId) -> Self {
        Dames {
            id: game_id.id,
            name: game_id.name,
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }
}

impl Game for Dames {
    fn render(&self) -> Markup {
        html! {
            "Game " (self.get_id())
        }
    }
}
