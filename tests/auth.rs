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
    TEST_LOCK.lock().unwrap_or_else(|p| p.into_inner())
}

async fn me_handler(headers: HeaderMap) -> Response {
    let auth = headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    let cookie = headers
        .get("cookie")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    if cookie.contains("qdiv0_access=new-token") || auth == "Bearer k-123" {
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
#[allow(clippy::await_holding_lock)]
async fn session_access_is_sent_as_cookie() {
    let _guard = lock();
    auth::reset();
    auth::set_access_token("session-token", 0);
    auth::set_refresh_token("refresh-token");
    let mut request = reqwest::Client::new()
        .get("https://example.invalid/v1/auth/me")
        .header("cookie", "other=kept; qdiv0_access=stale")
        .build()
        .unwrap();
    auth::inject(&mut request).await.unwrap();
    assert!(request.headers().get("authorization").is_none());
    let cookie = request.headers().get("cookie").unwrap().to_str().unwrap();
    assert!(cookie.contains("qdiv0_access=session-token"));
    assert!(cookie.contains("other=kept"));
    assert!(!cookie.contains("qdiv0_access=stale"));
    assert!(!cookie.contains("qdiv0_refresh"));
}

#[tokio::test]
#[allow(clippy::await_holding_lock)]
async fn login_and_refresh_rotate_session_cookies() {
    let _guard = lock();
    auth::reset();
    let calls = Arc::new(AtomicU32::new(0));
    let refresh_calls = calls.clone();
    let app = Router::new()
        .route(
            "/v1/auth/login",
            post(|| async {
                let mut headers = HeaderMap::new();
                headers.append(
                    "set-cookie",
                    "qdiv0_access=access-1; Max-Age=3600; Path=/; HttpOnly"
                        .parse()
                        .unwrap(),
                );
                headers.append(
                    "set-cookie",
                    "qdiv0_refresh=refresh-1; Path=/v1/auth/refresh; HttpOnly"
                        .parse()
                        .unwrap(),
                );
                (headers, axum::Json(json!({})))
            }),
        )
        .route(
            "/v1/auth/refresh",
            post(move |headers: HeaderMap| async move {
                let call = refresh_calls.fetch_add(1, Ordering::SeqCst);
                let cookie = headers.get("cookie").unwrap().to_str().unwrap();
                let expected = if call == 0 { "refresh-1" } else { "refresh-2" };
                assert!(cookie.contains(&format!("qdiv0_refresh={expected}")));
                let mut response_headers = HeaderMap::new();
                response_headers.append(
                    "set-cookie",
                    "qdiv0_access=access-2; Path=/; HttpOnly".parse().unwrap(),
                );
                response_headers.append(
                    "set-cookie",
                    "qdiv0_refresh=refresh-2; Path=/v1/auth/refresh; HttpOnly"
                        .parse()
                        .unwrap(),
                );
                (response_headers, axum::Json(json!({})))
            }),
        );
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
    let client = qdivzero::Client::new(&format!("http://{addr}"));
    auth::login(&client, "user@example.com", "secret")
        .await
        .unwrap();
    assert_eq!(auth::access_token(), "access-1");
    assert!(!auth::token_near_expiry());
    auth::refresh(&client).await.unwrap();
    assert_eq!(auth::access_token(), "access-2");
    auth::refresh(&client).await.unwrap();
    assert_eq!(calls.load(Ordering::SeqCst), 2);
}

#[tokio::test]
#[allow(clippy::await_holding_lock)] // serialization lock held across await
async fn injects_api_key_bearer() {
    let _guard = lock();
    auth::reset();
    let app = Router::new().route("/v1/auth/me", get(me_handler));
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });

    auth::set_api_key("k-123");
    let client = qdivzero::Client::new(&format!("http://{addr}"));
    let resp = client.get_auth_me().await.unwrap();
    assert_eq!(resp.status(), reqwest::StatusCode::OK);
}

#[tokio::test]
#[allow(clippy::await_holding_lock)] // serialization lock held across await
async fn refreshes_once_on_401_and_retries() {
    let _guard = lock();
    auth::reset();
    let refresh_calls = Arc::new(AtomicU32::new(0));
    let calls = refresh_calls.clone();
    let app = Router::new().route("/v1/auth/me", get(me_handler)).route(
        "/v1/auth/refresh",
        post(move |headers: HeaderMap| async move {
            calls.fetch_add(1, Ordering::SeqCst);
            let cookie = headers.get("cookie").unwrap().to_str().unwrap();
            assert!(cookie.contains("qdiv0_refresh=r1"));
            let mut response_headers = HeaderMap::new();
            response_headers.append(
                "set-cookie",
                "qdiv0_access=new-token; Path=/; HttpOnly".parse().unwrap(),
            );
            response_headers.append(
                "set-cookie",
                "qdiv0_refresh=r2; Path=/v1/auth/refresh; HttpOnly"
                    .parse()
                    .unwrap(),
            );
            (response_headers, axum::Json(json!({})))
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
#[allow(clippy::await_holding_lock)] // serialization lock held across await
async fn loads_credentials_file() {
    let _guard = lock();
    auth::reset();
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
