use axum::{
    Json, Router,
    response::IntoResponse,
    routing::{get, post},
};
use lettre::{Message, SmtpTransport, Transport};
use serde::Deserialize;

// Function that builds the Axum app (Reusable in tests)
pub fn create_app() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/send_email", post(send_email))
}

// Handler for `/`
async fn hello_world() -> impl IntoResponse {
    "Hello, World!"
}

#[derive(Deserialize)]
struct EmailRequest {
    to: String,
    message: String,
}

async fn send_email(Json(payload): Json<EmailRequest>) -> impl IntoResponse {
    let email = Message::builder()
        .from("your_email@example.com".parse().unwrap())
        .to(payload.to.parse().unwrap())
        .subject("Hello from Axum and Lettre")
        .body(payload.message)
        .unwrap();

    let mailer = SmtpTransport::builder_dangerous("smtp.example.com").build();

    match mailer.send(&email) {
        Ok(_) => "Email sent successfully".into_response(),
        Err(e) => format!("Failed to send email: {:?}", e).into_response(),
    }
}
