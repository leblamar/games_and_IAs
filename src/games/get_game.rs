use axum::extract::Path;
use maud::Markup;

use super::game_factory::GameFactory;

pub async fn game(Path(id): Path<String>) -> Markup {
    let game_factory = GameFactory { id };

    game_factory.create_game()
}
