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
    fn symbol(&self) -> char;
}

struct Add;
struct Sub;
struct Mul;
struct Div;

impl Operation for Add {
    fn calculate(&self, a: i32, b: i32) -> f64 {
        a as f64 + b as f64
    }

    fn symbol(&self) -> char {
        '+'
    }
}

impl Operation for Sub {
    fn calculate(&self, a: i32, b: i32) -> f64 {
        a as f64 - b as f64
    }

    fn symbol(&self) -> char {
        '-'
    }
}

impl Operation for Mul {
    fn calculate(&self, a: i32, b: i32) -> f64 {
        a as f64 * b as f64
    }

    fn symbol(&self) -> char {
        '*'
    }
}

impl Operation for Div {
    fn calculate(&self, a: i32, b: i32) -> f64 {
        a as f64 / b as f64
    }

    fn symbol(&self) -> char {
        '/'
    }
}

fn make_line(op: &dyn Operation, a: i32, b: i32) -> String {
    format!(
        "{} {} {} = {}",
        a,
        op.symbol(),
        b,
        op.calculate(a, b)
    )
}

fn calculate_all(a: i32, b: i32) -> String {
    let ops: [&dyn Operation; 4] = [&Add, &Sub, &Mul, &Div];

    ops.iter()
        .map(|op| make_line(*op, a, b))
        .collect::<Vec<_>>()
        .join("\n")
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
    let f_res = payload.first.trim().parse::<i32>();
    let s_res = payload.second.trim().parse::<i32>();

    match (f_res, s_res) {
        (Ok(f), Ok(s)) if f >= 0 && s >= 0 => {
            if s == 0 {
                ResultTemplate {
                    result: String::new(),
                    error: "Second number cannot be 0 (division by zero).".into(),
                }
            } else {
                ResultTemplate {
                    result: calculate_all(f, s),
                    error: String::new(),
                }
            }
        }
        _ => ResultTemplate {
            result: String::new(),
            error: "Only non-negative numeric values are allowed.".into(),
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

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();

    println!("Running: http://{}", addr);

    axum::serve(listener, app)
        .await
        .unwrap();
}