use askama::Template;
use axum::{Form, Router, routing::{get, post}};
use serde::Deserialize;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

#[derive(Template)]
#[template(path = "result.html")]
struct ResultTemplate {
    error: String,
    result: String,
    color: String,
}

#[derive(Deserialize)]
struct BmiForm {
    unit: Option<String>,
    weight_kg: Option<f64>,
    height_cm: Option<f64>,
    weight_lbs: Option<f64>,
    height_ft: Option<f64>,
    height_in: Option<f64>,
}

fn bmi_category(bmi: f64) -> (&'static str, &'static str) {
    if bmi < 18.5 {
        ("Underweight — see your doctor", "#3498db")
    } else if bmi < 25.0 {
        ("Normal weight", "#2ecc71")
    } else if bmi < 30.0 {
        ("Overweight — see your doctor", "#f39c12")
    } else {
        ("Obese — see your doctor", "#e74c3c")
    }
}

async fn index() -> IndexTemplate {
    IndexTemplate
}

async fn submit(Form(form): Form<BmiForm>) -> ResultTemplate {
    let unit = form.unit.as_deref().unwrap_or("metric");

    let bmi_result: Result<f64, &str> = match unit {
        "imperial" => {
            let weight = form.weight_lbs.unwrap_or(0.0);
            let feet = form.height_ft.unwrap_or(0.0);
            let inches = form.height_in.unwrap_or(0.0);
            let total_inches = feet * 12.0 + inches;
            if total_inches <= 0.0 || weight <= 0.0 {
                Err("Please enter valid height and weight.")
            } else {
                Ok(703.0 * weight / (total_inches * total_inches))
            }
        }
        _ => {
            let weight = form.weight_kg.unwrap_or(0.0);
            let height_m = form.height_cm.unwrap_or(0.0) / 100.0;
            if height_m <= 0.0 || weight <= 0.0 {
                Err("Please enter valid height and weight.")
            } else {
                Ok(weight / (height_m * height_m))
            }
        }
    };

    match bmi_result {
        Ok(bmi) => {
            let (category, color) = bmi_category(bmi);
            ResultTemplate {
                error: String::new(),
                result: format!("BMI: {:.1}  —  {}", bmi, category),
                color: color.to_string(),
            }
        }
        Err(e) => ResultTemplate {
            error: e.to_string(),
            result: String::new(),
            color: String::new(),
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
