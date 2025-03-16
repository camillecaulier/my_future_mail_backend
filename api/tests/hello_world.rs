use axum::body::Body;
use axum::body::to_bytes;
use axum::http::{Request, StatusCode};
use my_future_mail_backend::create_app;
use tower::ServiceExt; // Required for testing // Import app from `lib.rs`

#[tokio::test]
async fn test_hello_world() {
    let app = create_app();
    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), 1000).await.unwrap();
    assert_eq!(body, "Hello, World!");
}
