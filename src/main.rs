mod games;
mod home;
mod template_utils;

use axum::{routing::get, Router};
use home::home;
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use games::{get_game::game, get_game_template::game_template, get_games::games};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Initializing routes...");

    let api = Router::new()
        .nest_service("/static", ServeDir::new("static"))
        .route("/", get(home))
        .route("/games", get(games))
        .route("/game_template/:id", get(game_template))
        .route("/game/:id", get(game));

    let port: u16 = 8000;
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    tracing::info!(
        "Routes initialized, the server is now listening on port {}",
        port
    );

    axum::serve(listener, api).await.unwrap();
}
