use axum::response::IntoResponse;
use crate::html_template::html_template::HtmlTemplate;

use askama::Template;
pub async fn get_home() -> impl IntoResponse {
    let template = Home {};
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "index.html")]
struct Home;