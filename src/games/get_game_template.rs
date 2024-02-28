use axum::extract::Path;
use maud::{html, Markup};

use super::game_factory::GameFactory;
use crate::template_utils::{
    button_utils::button, htmx_instructions_builder::HtmxInstructionsBuilder, loader::loader,
    swap_opt::SwapOpt,
};

pub async fn game_template(Path(id): Path<String>) -> Markup {
    let game_factory = GameFactory { id };
    let game = game_factory.create_game_id().unwrap();

    let return_button_label = "Go back to other games";
    let button_htmx = HtmxInstructionsBuilder::new()
        .get("/")
        .push_url()
        .target("body")
        .swap(&SwapOpt::OUTER_HTML)
        .build();

    let get_board_game = format!("/game/{}", game.get_id());
    let board_game_loader_htmx = HtmxInstructionsBuilder::new()
        .get(get_board_game.as_str())
        .swap(&SwapOpt::OUTER_HTML)
        .build();

    html! {
        #game_header.relative.p-2 {
            #return_button_wrapper.absolute.top-0.right-0 {
                (button(return_button_label, button_htmx))
            }
            #game_title.flex.flex-row.justify-center {
                #game_title { (game.get_name()) }
            }
        }
        #board_game {
            (loader("board_game_loader", board_game_loader_htmx))
        }
    }
}
