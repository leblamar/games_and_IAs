pub enum SwapOpt {
    INNER_HTML,
    OUTER_HTML,
    BEFORE_BEGIN,
    AFTER_BEGIN,
    BEFORE_END,
    AFTER_END,
    DELETE,
    NONE
}

impl SwapOpt {
    pub fn as_str(&self) -> &str {
        match self {
            SwapOpt::INNER_HTML => "innerHTML",
            SwapOpt::OUTER_HTML => "outerHTML",
            SwapOpt::BEFORE_BEGIN => "beforebegin",
            SwapOpt::AFTER_BEGIN => "afterbegin",
            SwapOpt::BEFORE_END => "beforeend",
            SwapOpt::AFTER_END => "afterend",
            SwapOpt::DELETE => "delete",
            SwapOpt::NONE => "none"
        }
    }
}