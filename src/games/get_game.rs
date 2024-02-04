use axum::{response::IntoResponse, extract::Path};
use askama::Template;

use crate::html_template::html_template::HtmlTemplate;
use crate::games::game::Game;

pub async fn get_game(
    Path(id): Path<usize>
) -> impl IntoResponse {
    let games = vec![
        Game { id: 1, name: "Morpion".to_string() },
        Game { id: 2, name: "Puissance 4".to_string() },
        Game { id: 3, name: "Les Dames".to_string() }
    ];

    let game_template = games.iter()
        .find(|game| game.id == id)
        .map(GameTemplate::from)
        .unwrap();

    HtmlTemplate(game_template)
}

#[derive(Template)]
#[template(path = "game_template.html")]
struct GameTemplate {
    game: Game
}

impl From<&Game> for GameTemplate {
    fn from(value: &Game) -> Self {
        GameTemplate { game: value.clone() }
    }
}