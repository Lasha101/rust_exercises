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
    unit: String,
    shape: String,
    h_inner1: Option<String>,
    h_inner2: Option<String>,
    v_inner1: Option<String>,
    v_inner2: Option<String>,
}

struct Rectangle {
    width: f64, 
    height: f64,
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

struct Cyrcle {
    radius: f64,
}

impl Cyrcle {
    fn area(&self) -> f64 {
        Rectangle {
            width: self.radius,
            height: self.radius,
        }
        .area()
        *std::f64::consts::PI
    }
}

struct LShape {
    outer: Rectangle,
    curout: Rectangle,
}

impl LShape {
    fn area(&self) -> f64 {
        self.outer.area() - self.curout.area()
    }
}

fn num_of_gallons_to_paint(area: &f64, index: &i32) -> i32 {
    let result = *area / (*index as f64);
    result.ceil() as i32
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
    let f = payload.first.trim().parse::<f64>().unwrap_or(0.0);
    let s = payload.second.trim().parse::<f64>().unwrap_or(0.0);
    let unit = payload.unit.as_str();
    let shape = payload.shape.as_str();

    if f <= 0.0 || (shape != "cyrcle" && s <= 0.0) {
        return ResultTemplate {
            result: String::new(),
            error: "Please enter positive numeric values.".into(),
        }
        .into_response();
    }

    let area_val: f64;

    match shape {
        "rect" => {
            area_val = Rectangle { width: f, height: s }.area();
        }
        "cyrcle" => {
            area_val = Cyrcle { radius: f }.area();
        }
        "l-shape" => {
            let parse_opt = |opt: &Option<String>| opt.as_ref().and_then(|v| v.trim().parse::<f64>().ok()).unwrap_or(0.0);
            let h1 = parse_opt(&payload.h_inner1);
            let h2 = parse_opt(&payload.h_inner2);
            let v1 = parse_opt(&payload.v_inner1);
            let v2 = parse_opt(&payload.v_inner2);

            if (f - (h1 + h2)).abs() > 0.001 || (s - (v1 + v2)).abs() > 0.001 {
                return ResultTemplate {
                    result: String::new(),
                    error: format!("Validation failed: Outer dimensions must equal inner sums ({} = {} + {} and {} = {} + {}).", f, h1, h2, s, v1, v2),
                }.into_response();
            }
            area_val = LShape {
                outer: Rectangle { width: f, height: s },
                curout: Rectangle { width: h2, height: v1 },
            }.area();
        }
        _ => return ResultTemplate { result: String::new(), error: "Invalid shape.".into() }.into_response(),
    }

    let area_in_feet = if unit == "meters" {
        area_val / 0.09290304
    } else {
        area_val
    };

    const GALLON_COVERAGE: i32 = 350;
    let gallons = num_of_gallons_to_paint(&area_in_feet, &GALLON_COVERAGE);

    ResultTemplate {
        result: format!(
            "The area is {:.2} square {}.\nYou will need to purchase {} gallon(s) of paint.",
            area_val, unit, gallons
        ),
        error: String::new(),
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