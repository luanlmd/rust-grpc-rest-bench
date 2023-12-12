pub mod calculator { tonic::include_proto!("calculator"); }
use axum::{ routing::get, middleware::{self, Next}, http::{StatusCode, Request}, Json, Router, response::IntoResponse };

#[derive(Serialize)]
#[serde(remote = "Message")]
struct MessageDef {
    message: String
}

async fn root() -> (StatusCode, Json<MessageDef>)
{
    let response = MessageDef {
        message: "Hello from Rust".to_string()
    };

    (StatusCode::ACCEPTED, Json(response))
}

async fn hello() -> (StatusCode, Json<Message>) {
    let message = Message { message: "Hello from Rest".to_string() };
    (StatusCode::ACCEPTED, Json(message))
}

// async fn add(Json(payload): Json<CalculationSubjects>) -> (StatusCode, Json<CalculationResult>) {
//     let response = CalculationResult { value: payload.a + payload.b };
//     (StatusCode::OK, Json(response))
// }

#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/", get(hello))
    .layer(middleware::from_fn(log_requets));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    println!("Starting server on {:?}", listener);
    axum::serve(listener, app).await.unwrap();
}

async fn log_requets(req: Request<axum::body::Body>, next: Next) -> Result<impl IntoResponse, (StatusCode, String)> {
    println!("{} {}", req.method(), req.uri());
    Ok(next.run(req).await)
}
