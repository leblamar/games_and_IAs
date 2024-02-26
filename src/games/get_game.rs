use axum::extract::Path;
use maud::{Markup, html};

use super::game_factory::GameFactory;

pub async fn get_game(
    Path(id): Path<String>
) -> Markup {
    let game_factory = GameFactory { id };
    let game = game_factory.create_game_id().unwrap();
    //let game_html = game_factory.create_game_html().unwrap();

    html! {
        #game_header.flex.flex-row.justify-center {
            #game_title { (game.get_name()) }
            #return_button {}
        }
        div { "Game with id:" (game.get_id()) " and with name:" (game.get_name()) }
    }
}