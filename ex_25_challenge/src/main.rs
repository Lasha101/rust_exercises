use askama::Template;
use askama_axum::IntoResponse;
use axum::{
    extract::Form,
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use std::net::SocketAddr;

fn password_validator(input: &str) -> i32 {
    if input.is_empty() {
        return 0;
    }

    let len = input.chars().count();
    let has_digit = input.chars().any(|c| c.is_ascii_digit());
    let has_letter = input.chars().any(|c| c.is_ascii_alphabetic());
    let has_special = input.chars().any(|c| !c.is_ascii_alphanumeric());
    let all_digits = input.chars().all(|c| c.is_ascii_digit());
    let all_letters = input.chars().all(|c| c.is_ascii_alphabetic());

    if len < 8 {
        if all_digits {
            return 1; // Very Weak
        } else if all_letters {
            return 2; // Weak
        }
    } else {
        if has_letter && has_digit && has_special {
            return 4; // Very Strong
        } else if has_letter && has_digit {
            return 3; // Strong
        }
    }

    5 
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

#[derive(Template)]
#[template(path = "result.html")]
struct ResultTemplate {
    strength_text: String,
    strength_color: String,
    strength_width: i32, 
}

async fn index() -> IndexTemplate {
    IndexTemplate
}

#[derive(Deserialize)]
struct PasswordInput {
    password: String,
}

async fn check_password(Form(payload): Form<PasswordInput>) -> impl IntoResponse {
    let strength_indicator = password_validator(&payload.password);

    let (strength_text, strength_color, strength_width) = match strength_indicator {
        0 => ("Enter a password".to_string(), "#cccccc".to_string(), 0), // Grey for empty
        1 => ("Very Weak".to_string(), "#e74c3c".to_string(), 25), // Red
        2 => ("Weak".to_string(), "#f39c12".to_string(), 50), // Orange
        3 => ("Strong".to_string(), "#2ecc71".to_string(), 75), // Light Green
        4 => ("Very Strong".to_string(), "#27ae60".to_string(), 100), // Dark Green
        5 => ("Incomplete Pattern".to_string(), "#95a5a6".to_string(), 10), // Grey
        _ => ("Unknown Strength".to_string(), "#000000".to_string(), 0), // Should not happen
    };

    ResultTemplate {
        strength_text,
        strength_color,
        strength_width,
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/check-password", post(check_password));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Running: http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}
