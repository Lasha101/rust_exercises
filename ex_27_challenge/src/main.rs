use askama::Template;
use askama_axum::IntoResponse;
use axum::{
    extract::Form,
    routing::{get, post},
    Router,
};
use regex::Regex;
use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Deserialize)]
struct FormData {
    first_name: String,
    last_name: String,
    zip_code: String,
    employee_id: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

#[derive(Template)]
#[template(path = "result.html")]
struct ResultTemplate {
    messages: Vec<String>,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/validate", post(validate_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Listening on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> IndexTemplate {
    IndexTemplate
}

async fn validate_handler(Form(payload): Form<FormData>) -> impl IntoResponse {
    let messages = validate_input(&payload);
    ResultTemplate { messages }
}

fn validate_first_name(name: &str) -> Option<String> {
    if name.is_empty() {
        Some("The first name must be filled in.".to_string())
    } else if name.len() < 2 {
        Some(format!("\"{}\" is not a valid first name. It is too short.", name))
    } else {
        None
    }
}

fn validate_last_name(name: &str) -> Option<String> {
    if name.is_empty() {
        Some("The last name must be filled in.".to_string())
    } else if name.len() < 2 {
        Some(format!("\"{}\" is not a valid last name. It is too short.", name))
    } else {
        None
    }
}

fn validate_zip_code(zip: &str) -> Option<String> {
    if zip.is_empty() || !zip.chars().all(|c| c.is_ascii_digit()) {
        Some("The ZIP code must be numeric.".to_string())
    } else {
        None
    }
}

fn validate_employee_id(id: &str) -> Option<String> {
    let re = Regex::new(r"^[A-Z]{2}-\d{4}$").unwrap();
    if !re.is_match(id) {
        Some(format!("{} is not a valid ID.", id))
    } else {
        None
    }
}

fn validate_input(data: &FormData) -> Vec<String> {
    let mut errors = Vec::new();
    if let Some(e) = validate_first_name(&data.first_name) { errors.push(e); }
    if let Some(e) = validate_last_name(&data.last_name) { errors.push(e); }
    if let Some(e) = validate_zip_code(&data.zip_code) { errors.push(e); }
    if let Some(e) = validate_employee_id(&data.employee_id) { errors.push(e); }

    if errors.is_empty() {
        vec!["There were no errors found.".to_string()]
    } else {
        errors
    }
}