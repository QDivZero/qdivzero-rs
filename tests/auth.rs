use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};

use axum::{
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use serde_json::json;
use tokio::net::TcpListener;

use qdivzero::auth;

/// Serializes tests that mutate the global auth STATE and the HOME env var.
static TEST_LOCK: Mutex<()> = Mutex::new(());

fn lock() -> MutexGuard<'static, ()> {
    TEST_LOCK.lock().unwrap()
}

async fn me_handler(headers: HeaderMap) -> Response {
    let auth = headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    if auth == "Bearer new-token" || auth == "Bearer k-123" {
        axum::Json(json!({})).into_response()
    } else {
        (
            StatusCode::UNAUTHORIZED,
            axum::Json(json!({ "error": "unauthorized" })),
        )
            .into_response()
    }
}

#[tokio::test]
async fn injects_api_key_bearer() {
    let _guard = lock();
    let app = Router::new().route("/auth/me", get(me_handler));
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });

    auth::set_api_key("k-123");
    let client = qdivzero::Client::new(&format!("http://{addr}"));
    let resp = client.get_auth_me().await.unwrap();
    assert_eq!(resp.status(), reqwest::StatusCode::OK);
}

#[tokio::test]
async fn refreshes_once_on_401_and_retries() {
    let _guard = lock();
    let refresh_calls = Arc::new(AtomicU32::new(0));
    let calls = refresh_calls.clone();
    let app = Router::new().route("/auth/me", get(me_handler)).route(
        "/auth/refresh",
        post(move || async move {
            calls.fetch_add(1, Ordering::SeqCst);
            axum::Json(json!({ "access_token": "new-token" }))
        }),
    );
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });

    auth::set_access_token("stale", 0);
    auth::set_refresh_token("r1");
    let client = Arc::new(qdivzero::Client::new(&format!("http://{addr}")));

    let resp = auth::retry_after_refresh(&client, || {
        let client = client.clone();
        async move { client.get_auth_me().await }
    })
    .await
    .unwrap();
    assert_eq!(resp.status(), reqwest::StatusCode::OK);
    assert_eq!(refresh_calls.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn loads_credentials_file() {
    let _guard = lock();
    let dir = std::env::temp_dir().join(format!("qdivzero-rs-test-{}", std::process::id()));
    std::fs::create_dir_all(dir.join(".qdivzero")).unwrap();
    std::fs::write(
        dir.join(".qdivzero/credentials"),
        r#"{"access_token": "file-token"}"#,
    )
    .unwrap();
    std::env::set_var("HOME", &dir);
    auth::load_credentials();
    std::env::remove_var("HOME");
    std::fs::remove_dir_all(&dir).unwrap();
    assert_eq!(auth::access_token(), "file-token");
}
