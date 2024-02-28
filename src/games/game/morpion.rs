use maud::{html, Markup};

use super::game::Game;
use crate::games::game_id::GameId;

pub struct Morpion {
    id: String,
    name: String,
}

impl Morpion {
    pub fn new(game_id: GameId) -> Self {
        Morpion {
            id: game_id.id,
            name: game_id.name,
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }
}

impl Game for Morpion {
    fn render(&self) -> Markup {
        html! {
            "Game " (self.get_id())
        }
    }
}
