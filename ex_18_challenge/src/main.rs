use axum::{routing::{get, post}, Router, Form};
use askama::Template;
use askama_axum::IntoResponse;
use serde::Deserialize;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    result: String,
    error: String,
}

#[derive(Template)]
#[template(path = "result.html")]
struct ResultTemplate {
    result: String,
    error: String,
}

#[derive(Deserialize)]
struct ConvertForm {
    value: Option<String>,
    unit: Option<String>,
}

trait Calculation {
    fn calculate(&self, a: f64) -> f64;
}

struct CelsiusToFahrenheit;
struct FahrenheitToCelsius;
struct CelsiusToKelvin;
struct KelvinToCelsius;

impl Calculation for CelsiusToFahrenheit {
    fn calculate(&self, a: f64) -> f64 { (a * 9.0 / 5.0) + 32.0 }
}

impl Calculation for FahrenheitToCelsius {
    fn calculate(&self, a: f64) -> f64 { (a - 32.0) * 5.0 / 9.0 }
}

impl Calculation for CelsiusToKelvin {
    fn calculate(&self, a: f64) -> f64 { a + 273.15 }
}

impl Calculation for KelvinToCelsius {
    fn calculate(&self, a: f64) -> f64 { a - 273.15 }
}

fn convert(value: f64, from_unit: &str) -> String {
    let celsius = match from_unit {
        "fahrenheit" => FahrenheitToCelsius.calculate(value),
        "kelvin"     => KelvinToCelsius.calculate(value),
        _            => value,
    };
    format!(
        "Celsius:    {:.2}°C\nFahrenheit: {:.2}°F\nKelvin:     {:.2}K",
        celsius,
        CelsiusToFahrenheit.calculate(celsius),
        CelsiusToKelvin.calculate(celsius),
    )
}

async fn index() -> impl IntoResponse {
    IndexTemplate { result: String::new(), error: String::new() }
}

async fn submit(Form(form): Form<ConvertForm>) -> impl IntoResponse {
    let value_str = form.value.unwrap_or_default();
    let unit = form.unit.unwrap_or_else(|| "celsius".to_string());

    match value_str.trim().parse::<f64>() {
        Ok(value) => ResultTemplate { result: convert(value, &unit), error: String::new() },
        Err(_) => ResultTemplate {
            result: String::new(),
            error: if value_str.trim().is_empty() {
                String::new()
            } else {
                "Cannot calculate value. Please try again.".to_string()
            },
        },
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/submit", post(submit));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
