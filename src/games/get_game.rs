use axum::{response::IntoResponse, Json};
use askama::Template;
use serde::Deserialize;

use crate::html_template::html_template::HtmlTemplate;

pub async fn get_game(
    Json(requested_game): Json<GameRequest>
) -> impl IntoResponse {
    let games = vec![
        Game { id: 1, name: "Morpion".to_string() },
        Game { id: 2, name: "Puissance 4".to_string() },
        Game { id: 3, name: "Les Dames".to_string() }
    ];

    let game_template = games.iter()
        .find(|game| game.id == requested_game.id)
        .map(GameTemplate::from)
        .unwrap();

    HtmlTemplate(game_template)
}

#[derive(Deserialize)]
pub struct GameRequest {
    id: usize
}

struct Game {
    id: usize,
    name: String,
}

#[derive(Template)]
#[template(path = "game_template.html")]
struct GameTemplate {
    current_game: String
}

impl From<&Game> for GameTemplate {
    fn from(value: &Game) -> Self {
        GameTemplate { current_game: value.name.clone() }
    }
}