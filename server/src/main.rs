use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

mod types;
use types::User;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
        // .allow_credentials();

    let app = Router::new()
        //routes
        .route("/", get(|| async { "Hello there." })) 
        .route("/users", get(get_users))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 5000));
    println!("Server is Running and Listening on {} 🚀", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_users() -> impl IntoResponse {
    let users = vec![
        User {
            id: 1,
            name: "Bruce".to_string(),
            email: "bruce@wayne.com".to_string(),
        },
        User {
            id: 2,
            name: "Clark".to_string(),
            email: "clark@dailyplanet.com".to_string(),
        },
    ];

    (StatusCode::OK, Json(users))
}
