mod home;
mod html_template;
mod games;

use axum::{
    routing::get,
    Router
};

use home::home::get_home;
use games::{get_games::get_games, get_game::get_game};

#[tokio::main]
async fn main() {
    println!("Let's start the server !!!");
    let api = Router::new()
        .route("/home", get(get_home))
        .route("/api/get_games", get(get_games))
        .route("/api/get_game", get(get_game));
    let port: u16 = 8000;
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, api).await.unwrap();
}
