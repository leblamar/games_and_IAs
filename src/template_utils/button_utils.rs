use maud::{html, Markup};
use super::htmx_instructions::HtmxInstructions;

pub fn button(
    label: &str,
    htmx_instructions: HtmxInstructions
) -> Markup {
    html! {
        button .p-2.m-2.rounded.border-2.border-blue-500
            hx-get=[htmx_instructions.get_url()]
            hx-push-url=[htmx_instructions.should_push_url()]
            hx-target=[htmx_instructions.target_id()]
            hx-swap=[htmx_instructions.swap_option()] {
                (label)
        }
    }
}