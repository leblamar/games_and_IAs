use maud::Markup;

use crate::template_utils::{
    base_utils::base, htmx_instructions_builder::HtmxInstructionsBuilder, loader::loader,
    swap_opt::SwapOpt,
};

pub async fn home() -> Markup {
    let content_htmx = HtmxInstructionsBuilder::new()
        .get("games")
        .swap(&SwapOpt::OUTER_HTML)
        .build();
    let content = loader("games_loader", content_htmx);

    base("Games and IAs", content)
}
