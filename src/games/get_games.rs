use axum::response::IntoResponse;
use askama::Template;

use crate::html_template::html_template::HtmlTemplate;

pub async fn get_games() -> impl IntoResponse {
    let template = Games {
        games: vec![
            Game { id: 1, name: "Morpion".to_string() },
            Game { id: 2, name: "Puissance 4".to_string() },
            Game { id: 3, name: "Les Dames".to_string() }
        ]
    };
    HtmlTemplate(template)
}

struct Game {
    id: usize,
    name: String,
}

#[derive(Template)]
#[template(path = "games.html")]
struct Games {
    games: Vec<Game>
}