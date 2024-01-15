mod home;
mod html_template;

use axum::{
    routing::get,
    Router
};

use home::home::get_home;

#[tokio::main]
async fn main() {
    println!("Let's start the server !!!");
    let api = Router::new()
        .route("/home", get(get_home))
        .route("/api/get_test_1", get(|| async { "Hello 1 !!!"}))
        .route("/api/get_test_2", get(|| async { "Hello 2 !!!"}));
    let port: u16 = 8000;
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, api).await.unwrap();
}
