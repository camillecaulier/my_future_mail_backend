use axum::{Router, response::IntoResponse, routing::get};

// Function that builds the Axum app (Reusable in tests)
pub fn create_app() -> Router {
    Router::new().route("/", get(hello_world))
}

// Handler for `/`
async fn hello_world() -> impl IntoResponse {
    "Hello, World!"
}
