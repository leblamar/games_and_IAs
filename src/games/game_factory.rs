use maud::Markup;

use crate::games::game_id::GameId;

use super::game::{dames::Dames, game::Game, morpion::Morpion, puissance4::Puissance4};

pub struct GameFactory {
    pub id: String,
}

impl GameFactory {
    pub fn get_games() -> Vec<GameId> {
        vec![
            GameId {
                id: "morpion".to_string(),
                name: "Morpion".to_string(),
            },
            GameId {
                id: "puissance4".to_string(),
                name: "Puissance 4".to_string(),
            },
            GameId {
                id: "lesdames".to_string(),
                name: "Les Dames".to_string(),
            },
        ]
    }

    pub fn create_game_id(&self) -> Option<GameId> {
        GameFactory::get_games()
            .into_iter()
            .find(|game| game.id == self.id)
    }

    pub fn create_game(&self) -> Markup {
        let game_id = self.create_game_id().unwrap();

        match self.id.as_str() {
            "morpion" => Morpion::new(game_id).render(),
            "puissance4" => Puissance4::new(game_id).render(),
            "lesdames" => Dames::new(game_id).render(),
            _ => panic!("Test"),
        }
    }
}
