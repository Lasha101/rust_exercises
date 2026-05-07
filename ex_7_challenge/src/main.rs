use askama::Template;
use askama_axum::IntoResponse;
use axum::{
    extract::Form,
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Debug, Deserialize)]
struct UserInput {
    first: String,
    second: String,
}

trait Operation {
    fn calculate(&self, a: i32, b: i32) -> f64;
}

struct Mul;

impl Operation for Mul {
    fn calculate(&self, a: i32, b: i32) -> f64 {
        a as f64 * b as f64
    }
}

fn make_line(a: f64) -> String {
    format!("The area is \n{a} square feet")
}

fn calculate_area(a: i32, b: i32) -> String {
    let op: &dyn Operation = &Mul;
    let result = op.calculate(a, b);
    make_line(result)
}

#[derive(Template)]
#[template(path = "index.html")]
struct PageTemplate {
    result: String,
    error: String,
}

#[derive(Template)]
#[template(path = "result.html")]
struct ResultTemplate {
    result: String,
    error: String,
}

async fn show_form() -> impl IntoResponse {
    PageTemplate {
        result: String::new(),
        error: String::new(),
    }
}

async fn handle_submit(Form(payload): Form<UserInput>) -> impl IntoResponse {
    let f_str = payload.first.trim();
    let s_str = payload.second.trim();

    if f_str.is_empty() || s_str.is_empty() {
        return ResultTemplate {
            result: String::new(),
            error: "Both fields are required.".into(),
        }
        .into_response();
    }

    match (f_str.parse::<i32>(), s_str.parse::<i32>()) {
        (Ok(f), Ok(s)) if f > 0 && s > 0 => ResultTemplate {
            result: calculate_area(f, s),
            error: String::new(),
        },
        _ => ResultTemplate {
            result: String::new(),
            error: "Only positive numeric values are allowed.".into(),
        },
    }
    .into_response()
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(show_form))
        .route("/submit", post(handle_submit));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Running: http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}