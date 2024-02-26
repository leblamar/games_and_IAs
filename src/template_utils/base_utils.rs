use maud::{html, Markup, DOCTYPE};

fn head(title: &str) -> Markup {
    html! {
        head {
            script src="static/scripts/htmx.min.js" {}
            script src="static/scripts/tailwind.min.js" {}
            title { (title) }
        }
    }
}

pub fn base(title: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html {
            (head(title))
            body {
                (content)
            }
        }
    }
}
