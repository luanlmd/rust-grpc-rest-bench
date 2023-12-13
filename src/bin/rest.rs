use serde_derive::{Serialize, Deserialize};
use axum::{ routing::{get, post}, middleware::{self, Next}, http::{StatusCode, Request}, Json, Router, response::IntoResponse };


#[derive(Serialize)]
struct Message {
    message: String
}

#[derive(Deserialize)]
struct CalculationSubjects {
    a: i32,
    b: i32
}

#[derive(Serialize)]
struct CalculationResult {
    value: i32
}

async fn log_requets(req: Request<axum::body::Body>, next: Next) -> Result<impl IntoResponse, (StatusCode, String)> {
    println!("{} {}", req.method(), req.uri());
    Ok(next.run(req).await)
}

async fn hello() -> (StatusCode, Json<Message>)
{
    let response = Message { message: "Hello from Rest".into() };
    (StatusCode::ACCEPTED, Json(response))
}

async fn add(Json(payload): Json<CalculationSubjects>) -> (StatusCode, Json<CalculationResult>) {
    let response = CalculationResult { value: payload.a + payload.b };
    (StatusCode::OK, Json(response))
}

async fn subtract(Json(payload): Json<CalculationSubjects>) -> (StatusCode, Json<CalculationResult>) {
    let response = CalculationResult {  value: payload.a - payload.b };
    (StatusCode::OK, Json(response))
}

async fn multiply(Json(payload): Json<CalculationSubjects>) -> (StatusCode, Json<CalculationResult>) {
    let response = CalculationResult { value: payload.a * payload.b };
    (StatusCode::OK, Json(response))
}

async fn divide(Json(payload): Json<CalculationSubjects>) -> (StatusCode, Json<CalculationResult>) {
    let response = CalculationResult { value: payload.a / payload.b };
    (StatusCode::OK, Json(response))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/", get(hello))
    .route("/add", post(add))
    .route("/subtract", post(subtract))
    .route("/multiply", post(multiply))
    .route("/divide", post(divide))
    .layer(middleware::from_fn(log_requets));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    println!("Starting server on {:?}", listener);
    axum::serve(listener, app).await.unwrap();
}
