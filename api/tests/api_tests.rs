use axum::{body::Body, http::{Request, StatusCode}};
use tower::ServiceExt; // for `oneshot` method
use my_future_mail_backend::create_app;

#[tokio::test]
async fn test_hello_world() {
    let app = create_app();

    let response = app.oneshot(Request::new(Body::empty())).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    assert_eq!(body, "Hello, World!");
}

#[tokio::test]
async fn test_send_email() {
    let app = create_app();

    let payload = serde_json::json!({
        "to": "recipient@example.com",
        "message": "Test email message"
    });

    let response = app.oneshot(
        Request::post("/send_email")
            .header("content-type", "application/json")
            .body(Body::from(payload.to_string()))
            .unwrap()
    ).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    assert_eq!(body, "Email sent successfully");
}
