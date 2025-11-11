use std::{str::FromStr, sync::Arc};

use rug::Integer;
use rust_test::prime_generator::{genarator, prime_check, prime_gen};
use axum::{
    extract::State, response::Html, routing::{get, post}, Json, Router
};
use serde::{Deserialize, Serialize};
use tower_http::services::ServeDir;
use tera::{Tera, Context};


#[tokio::main]
async fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() > 1 {
        let prime = genarator(args);
        println!("Generated prime number: {}", prime);
        std::process::exit(0);
    }

    println!("Starting Server at http://127.0.0.1:8080");

    // axum server setup

    let static_files = ServeDir::new("static");
    let tera = Arc::new(Tera::new("templates/**/*").expect("Template parsing failed"));

    // build our application with a route
    let app = Router::new()
        .route("/", get(home))
        .route("/generator", get(prime_genetator))
        .route("/api/generator", post(compute_prime))
        .route("/api/checker", post(check_prime))
        .nest_service("/static", static_files)
        .with_state(tera);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();

}

async fn home(State(tera) : State<Arc<Tera>>) -> Html<String> {
    let html = tera.render("index.html", &Context::new())
        .unwrap_or_else(|_| "Error rendering template".to_string());
    Html(html)
}

async fn prime_genetator(
    State(tera): State<Arc<Tera>>,
) -> Html<String> {
    let html = tera.render("generator.html", &Context::new()).unwrap();
    Html(html)
}

#[derive(Deserialize)]
struct GenerationInput {
    digits: String,
    reps: String,
}

#[derive(Serialize)]
struct ComputeResult {
    result: String,
}

async fn compute_prime( Json(payload) : Json<GenerationInput> ) -> Json<ComputeResult> {
    let digits = payload.digits.trim().parse::<u32>().unwrap_or(10);
    let reps = payload.reps.trim().parse::<u32>().unwrap_or(20);
    let result = prime_gen(digits, reps);
    Json(ComputeResult {
        result: result.to_string(),
    })
}

#[derive(Deserialize)]
struct CheckingInput {
    num: String,
}

async fn check_prime(Json(payload): Json<CheckingInput>) -> Json<ComputeResult> {
    let num = Integer::from_str(payload.num.trim());
    let result = match num {
        Ok(n) => {
            if prime_check(&n, 20) {
                "It's a Prime".to_string()
            } else {
                "It's Not a Prime".to_string()
            }
        },
        Err(_) => "Invalid number".to_string(),
    };
    Json(ComputeResult {
        result,
    })
}
