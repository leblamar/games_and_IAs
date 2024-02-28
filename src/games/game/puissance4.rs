use maud::{html, Markup};

use super::game::Game;
use crate::games::game_id::GameId;

pub struct Puissance4 {
    id: String,
    name: String,
}

impl Puissance4 {
    pub fn new(game_id: GameId) -> Self {
        Puissance4 {
            id: game_id.id,
            name: game_id.name,
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }
}

impl Game for Puissance4 {
    fn render(&self) -> Markup {
        html! {
            "Game " (self.get_id())
        }
    }
}
