use axum::{
    Router, 
    extract::Form, 
    routing::{get, post}
};
use askama_axum::IntoResponse;
use askama::Template;
use serde::Deserialize;
use validator::Validate;
use std::net::SocketAddr;


fn is_alphabetical(s: &str) -> Result<(), validator::ValidationError> {
    if s.chars().all(|c| c.is_alphabetic()) {
        Ok(())
    } else {
        Err(validator::ValidationError::new("Input must only contain letters"))
    }
}


#[derive(Deserialize, Validate)]
struct UserInput{
    #[validate(custom(function="is_alphabetical"))]
    row_text: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct HelloTemplate {
    symbol_count: usize,
}

async fn handle_request(Form(payload): Form<UserInput>) -> impl IntoResponse {
    match payload.validate() {
        Ok(_) => {
            HelloTemplate {
                symbol_count: payload.row_text.chars().count(),
            }.into_response() // This uses the askama_axum trait
        }
        Err(e) => format!("Validation failed: {}", e).into_response(),
    }
}

async fn show_form() -> impl IntoResponse {
    HelloTemplate { symbol_count: 0 }.into_response()
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(show_form))
        .route("/submit", post(handle_request));
    let addr = SocketAddr::from(([127,0,0,1], 3000));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Server running at http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}