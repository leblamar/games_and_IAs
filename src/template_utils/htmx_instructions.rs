pub struct HtmxInstructions<'a> {
    pub get: Option<&'a str>,
    pub push_url: Option<&'a str>,
    pub target: Option<&'a str>,
    pub swap: Option<&'a str>,
    pub trigger: Option<&'a str>,
}

impl HtmxInstructions<'_> {
    pub fn get_url(&self) -> Option<&str> {
        self.get
    }

    pub fn should_push_url(&self) -> Option<&str> {
        self.push_url
    }

    pub fn target_id(&self) -> Option<&str> {
        self.target
    }

    pub fn swap_option(&self) -> Option<&str> {
        self.swap
    }

    pub fn trigger_option(&self) -> Option<&str> {
        self.trigger
    }
}

