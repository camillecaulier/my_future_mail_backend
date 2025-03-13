use axum::http::{Request, StatusCode};
use hyper::Body;
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

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    assert_eq!(body, "Hello, World!");
}
