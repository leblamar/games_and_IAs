use maud::{html, Markup};

use crate::games::game_factory::GameFactory;
use crate::template_utils::{
    button_utils::button, htmx_instructions_builder::HtmxInstructionsBuilder,
};

use super::game_id::GameId;

fn button_game(game: GameId) -> Markup {
    let get = format!("game_template/{}", game.get_id());
    let game_instructions = HtmxInstructionsBuilder::new()
        .get(get.as_str())
        .push_url()
        .target("#games_nav_bar")
        .swap("outerHTML")
        .build();

    html! {
        (button(game.get_name(), game_instructions))
    }
}

pub async fn get_games() -> Markup {
    let games = GameFactory::get_games();

    html! {
        #games_nav_bar.flex.flex-row.justify-center {
            @for game in games {
                (button_game(game))
            }
        }
    }
}
