use std::collections::HashMap;
use axum::{extract::Form, routing::{get, post}, Router};
use askama::Template;
use serde::Deserialize;
use tokio::net::TcpListener;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    error: String,
    result: String,
}

#[derive(Template)]
#[template(path = "result.html")]
struct ResultTemplate {
    error: String,
    result: String,
}

#[derive(Deserialize)]
struct BacForm {
    unit: Option<String>,
    weight: Option<String>,
    gender: Option<String>,
    drinks: Option<String>,
    abv: Option<String>,
    hours: Option<String>,
    state: Option<String>,
}

fn state_bac_limits() -> HashMap<String, f64> {
    let mut m = HashMap::new();
    for s in &[
        "alabama", "alaska", "arizona", "arkansas", "california", "colorado",
        "connecticut", "delaware", "florida", "georgia", "hawaii", "idaho",
        "illinois", "indiana", "iowa", "kansas", "kentucky", "louisiana",
        "maine", "maryland", "massachusetts", "michigan", "minnesota",
        "mississippi", "missouri", "montana", "nebraska", "nevada",
        "new hampshire", "new jersey", "new mexico", "new york",
        "north carolina", "north dakota", "ohio", "oklahoma", "oregon",
        "pennsylvania", "rhode island", "south carolina", "south dakota",
        "tennessee", "texas", "vermont", "virginia", "washington",
        "west virginia", "wisconsin", "wyoming",
    ] {
        m.insert(s.to_string(), 0.08);
    }
    m.insert("utah".to_string(), 0.05);
    m
}

fn blood_alc_calculator(weight: f64, gender: f64, drinks: f64, abv: f64, hours: f64) -> f64 {
    (drinks * abv * 5.14) / (weight * gender) - 0.015 * hours
}

async fn index() -> IndexTemplate {
    IndexTemplate { error: String::new(), result: String::new() }
}

async fn submit(Form(data): Form<BacForm>) -> ResultTemplate {
    let parse = |s: Option<String>| -> Option<f64> {
        s.as_deref().and_then(|v| v.trim().parse().ok())
    };

    let weight_raw = match parse(data.weight) {
        Some(v) if v > 0.0 => v,
        _ => return ResultTemplate { error: String::new(), result: String::new() },
    };

    let unit = data.unit.unwrap_or_default();
    let weight_lbs = if unit == "metric" { weight_raw * 2.20462 } else { weight_raw };

    let gender: f64 = match data.gender.as_deref().unwrap_or("").trim() {
        "m" => 0.73,
        "f" => 0.66,
        _ => return ResultTemplate { error: String::new(), result: String::new() },
    };

    let drinks = match parse(data.drinks) {
        Some(v) => v,
        None => return ResultTemplate { error: String::new(), result: String::new() },
    };

    let abv = match parse(data.abv) {
        Some(v) if v > 0.0 => v,
        _ => return ResultTemplate { error: String::new(), result: String::new() },
    };

    let hours = match parse(data.hours) {
        Some(v) => v,
        None => return ResultTemplate { error: String::new(), result: String::new() },
    };

    let state = data.state.unwrap_or_default().trim().to_lowercase();
    if state.is_empty() {
        return ResultTemplate { error: String::new(), result: String::new() };
    }

    let limit = match state_bac_limits().get(&state).copied() {
        Some(l) => l,
        None => return ResultTemplate {
            error: format!("State '{}' not found.", state),
            result: String::new(),
        },
    };

    let bac = blood_alc_calculator(weight_lbs, gender, drinks, abv, hours);
    let status = if bac >= limit {
        format!("NOT legal to drive (limit: {:.3})", limit)
    } else {
        format!("Legal to drive (limit: {:.3})", limit)
    };

    ResultTemplate {
        error: String::new(),
        result: format!("BAC: {:.4}\nStatus: {}", bac, status),
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/submit", post(submit));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
