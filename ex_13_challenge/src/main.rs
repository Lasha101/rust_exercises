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
    principal: String,
    rate: String,
    years: String,
    compounding: String,
}


fn make_line(principal: f64, rate: f64, years: f64, per_year: f64, final_amount: f64) -> String {
    let years_as_int = years as i32;
    let rounded_initial_amount = (principal * 100.0).round()/100.0;
    format!("You will need investing ${} at {}% for {} years\n\
              compounded {} times per year to get ${}.",
              rounded_initial_amount, rate, years_as_int, per_year, final_amount)
}

fn calc_compound_interest(final_amount: &f64,
                        interest: &f64, 
                        years: &f64, 
                        per_year: &f64) -> String {
    let principal = final_amount / (1.0 + (interest/100.0)/per_year).powf(per_year*years);
    make_line(principal, *interest, *years, *per_year, *final_amount)
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
    let p_res = payload.principal.trim().parse::<f64>();
    let y_res = payload.years.trim().parse::<f64>();
    let r_res = payload.rate.trim().parse::<f64>();
    let c_res = payload.compounding.trim().parse::<f64>();
    match (p_res, r_res, y_res, c_res) {
        (Ok(p), Ok(r), Ok(y), Ok(c)) if p > 0.0 && r > 0.0 && y > 0.0 && c > 0.0 => {
            ResultTemplate {
                result: calc_compound_interest(&p, &r, &y, &c),
                error: String::new(),
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