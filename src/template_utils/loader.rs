use maud::{html, Markup};
use super::htmx_instructions::HtmxInstructions;

pub fn loader(id: &str, loading_instruction: HtmxInstructions) -> Markup {
    html! {
        #(id)
            hx-get=[loading_instruction.get_url()] 
            hx-target=[loading_instruction.target_id()]
            hx-push_url=[loading_instruction.should_push_url()]
            hx-swap=[loading_instruction.swap_option()]
            hx-trigger="load" {
            "Loading..."
        }
    }
}