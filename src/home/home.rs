use maud::{html, Markup};

use crate::template_utils::base_utils::base;

pub async fn get_home() -> Markup {
    let content = html! {
        div hx-get="api/get_games" hx-trigger="load" hx-swap="outerHTML" {}
    };

    base("Games and IAs", content)
}