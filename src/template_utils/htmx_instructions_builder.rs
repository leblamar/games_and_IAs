use super::{htmx_instructions::HtmxInstructions, swap_opt::SwapOpt};

pub struct HtmxInstructionsBuilder<'a> {
    get_url_opt: Option<&'a str>,
    push_url_opt: Option<bool>,
    target_id_opt: Option<&'a str>,
    swap_opt: Option<&'a SwapOpt>,
    trigger_opt: Option<&'a str>,
}

impl<'a> HtmxInstructionsBuilder<'a> {
    pub fn new() -> HtmxInstructionsBuilder<'a> {
        HtmxInstructionsBuilder {
            get_url_opt: None,
            push_url_opt: None,
            target_id_opt: None,
            swap_opt: None,
            trigger_opt: None,
        }
    }

    pub fn get(&mut self, get_url: &'a str) -> &mut HtmxInstructionsBuilder<'a> {
        self.get_url_opt = Some(get_url);
        self
    }

    pub fn push_url(&mut self) -> &mut HtmxInstructionsBuilder<'a> {
        self.push_url_opt = Some(true);
        self
    }

    pub fn target(&mut self, target_id: &'a str) -> &mut HtmxInstructionsBuilder<'a> {
        self.target_id_opt = Some(target_id);
        self
    }

    pub fn swap(&mut self, swap_opt: &'a SwapOpt) -> &mut HtmxInstructionsBuilder<'a> {
        self.swap_opt = Some(swap_opt);
        self
    }

    pub fn trigger(&mut self, trigger_opt: &'a str) -> &mut HtmxInstructionsBuilder<'a> {
        self.trigger_opt = Some(trigger_opt);
        self
    }

    pub fn build(&self) -> HtmxInstructions<'a> {
        HtmxInstructions {
            get: self.get_url_opt,
            push_url: self.push_url_opt.map(|push_url| {
                if push_url {
                    "true"
                } else {
                    "false"
                }
            }),
            target: self.target_id_opt,
            swap: self.swap_opt.map(SwapOpt::as_str),
            trigger: self.trigger_opt,
        }
    }
}
