use std::io::Read;

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use bytes::Bytes;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(body)).route("/html", get(ok));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn body() -> impl IntoResponse {
    let mut file = std::fs::File::open("Alarm1.mp3").unwrap();
    let mut buf = Vec::new();

    file.read_to_end(&mut buf).unwrap();
    println!("{:?}", buf);

    let response = Bytes::from(buf);
    println!("{:?}", response);
    (StatusCode::CREATED, response)
}

async fn ok() -> impl IntoResponse {
    let html ="<h1>Hello</h1>");

    (StatusCode::OK, html)
}
