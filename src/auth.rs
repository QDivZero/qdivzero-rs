//! Authentication state for the QDivZero API client (hand-written, not generated).

use std::future::Future;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use progenitor_client::Error;

pub struct AuthInner {
    pub api_key: String,
    pub access_token: String,
    pub refresh_token: String,
    pub email: String,
    pub password: String,
    pub access_token_expiry: AtomicU64, // unix seconds; 0 = unknown
}

pub static STATE: Mutex<AuthInner> = Mutex::new(AuthInner {
    api_key: String::new(),
    access_token: String::new(),
    refresh_token: String::new(),
    email: String::new(),
    password: String::new(),
    access_token_expiry: AtomicU64::new(0),
});

impl Default for AuthInner {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            access_token: String::new(),
            refresh_token: String::new(),
            email: String::new(),
            password: String::new(),
            access_token_expiry: AtomicU64::new(0),
        }
    }
}

/// Resets all authentication state (used by tests).
#[doc(hidden)]
pub fn reset() {
    *STATE.lock().unwrap() = AuthInner::default();
}

/// Pre-hook injected by the generated client before every request.
pub async fn inject(request: &mut reqwest::Request) -> Result<(), String> {
    let inner = STATE.lock().unwrap();
    let value = if !inner.access_token.is_empty() {
        format!("Bearer {}", inner.access_token)
    } else if !inner.api_key.is_empty() {
        format!("Bearer {}", inner.api_key)
    } else {
        return Ok(());
    };
    if let Ok(header) = reqwest::header::HeaderValue::from_str(&value) {
        request
            .headers_mut()
            .insert(reqwest::header::AUTHORIZATION, header);
    } else {
        return Err("invalid Authorization header value".to_string());
    }
    Ok(())
}

/// Sets the API key used as `Authorization: Bearer <key>`.
pub fn set_api_key(key: impl Into<String>) {
    STATE.lock().unwrap().api_key = key.into();
}

/// Sets an access token (and optionally its unix-seconds expiry; 0 = unknown).
pub fn set_access_token(token: impl Into<String>, expiry_unix: u64) {
    let mut inner = STATE.lock().unwrap();
    inner.access_token = token.into();
    inner
        .access_token_expiry
        .store(expiry_unix, Ordering::SeqCst);
}

/// Sets the refresh token used for automatic refresh.
pub fn set_refresh_token(token: impl Into<String>) {
    STATE.lock().unwrap().refresh_token = token.into();
}

/// Returns the current access token.
pub fn access_token() -> String {
    STATE.lock().unwrap().access_token.clone()
}

/// Authenticates with email/password and stores the returned tokens.
pub async fn login(
    client: &crate::Client,
    email: &str,
    password: &str,
) -> Result<(), Error<crate::types::ErrorResponse>> {
    let body = crate::types::LoginRequest {
        email: email.to_string(),
        password: password.to_string(),
        cf_turnstile_response: None,
        totp_code: None,
    };
    let resp = client.post_auth_login(&body).await?;
    let tokens = resp.into_inner();
    let access = tokens
        .access_token
        .ok_or_else(|| Error::Custom("login: empty access token".to_string()))?;
    let expiry = tokens.access_token_expiry.unwrap_or(0).max(0) as u64;
    let mut inner = STATE.lock().unwrap();
    inner.access_token = access;
    inner.access_token_expiry.store(expiry, Ordering::SeqCst);
    if let Some(rt) = tokens.refresh_token {
        inner.refresh_token = rt;
    }
    Ok(())
}

/// Refreshes the access token (refresh token first, then credentials).
pub async fn refresh(client: &crate::Client) -> Result<(), Error<crate::types::ErrorResponse>> {
    let (refresh_token, email, password) = {
        let inner = STATE.lock().unwrap();
        (
            inner.refresh_token.clone(),
            inner.email.clone(),
            inner.password.clone(),
        )
    };
    if !refresh_token.is_empty() {
        let body = crate::types::RefreshRequest { refresh_token };
        let resp = client.post_auth_refresh(&body).await?;
        let tokens = resp.into_inner();
        let access = tokens
            .access_token
            .ok_or_else(|| Error::Custom("refresh: empty access token".to_string()))?;
        let expiry = tokens.access_token_expiry.unwrap_or(0).max(0) as u64;
        let mut inner = STATE.lock().unwrap();
        inner.access_token = access;
        inner.access_token_expiry.store(expiry, Ordering::SeqCst);
        return Ok(());
    }
    if !email.is_empty() && !password.is_empty() {
        return login(client, &email, &password).await;
    }
    Err(Error::Custom(
        "no refresh token or credentials available".to_string(),
    ))
}

/// Returns true when the access token has less than 30 seconds of life left.
pub fn token_near_expiry() -> bool {
    let inner = STATE.lock().unwrap();
    let expiry = inner.access_token_expiry.load(Ordering::SeqCst);
    if expiry == 0 {
        return false;
    }
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    expiry.saturating_sub(now) < 30
}

/// Runs the closure; on a 401 error response, refreshes once and re-runs it.
pub async fn retry_after_refresh<T, F, Fut>(
    client: &crate::Client,
    f: F,
) -> Result<T, Error<crate::types::ErrorResponse>>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<T, Error<crate::types::ErrorResponse>>>,
{
    match f().await {
        Ok(v) => Ok(v),
        Err(Error::ErrorResponse(e)) if e.status() == reqwest::StatusCode::UNAUTHORIZED => {
            // The closure must be callable again; the caller passes a cloneable
            // client (e.g. Arc<Client>) captured by the closure.
            refresh(client).await?;
            f().await
        }
        Err(e) => Err(e),
    }
}

/// Loads ~/.qdivzero/credentials (JSON: email/password/access_token/refresh_token)
/// and applies the values to the auth state. A missing file is a no-op.
pub fn load_credentials() {
    let path = std::env::var("HOME")
        .map(|h| {
            std::path::Path::new(&h)
                .join(".qdivzero")
                .join("credentials")
        })
        .unwrap_or_else(|_| std::path::PathBuf::from(".qdivzero/credentials"));
    let raw = match std::fs::read_to_string(&path) {
        Ok(r) => r,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return,
        Err(e) => panic!("qdivzero: read credentials: {e}"),
    };
    let parsed: serde_json::Value =
        serde_json::from_str(&raw).expect("qdivzero: parse credentials");
    let mut inner = STATE.lock().unwrap();
    if let Some(v) = parsed.get("email").and_then(|v| v.as_str()) {
        inner.email = v.to_string();
    }
    if let Some(v) = parsed.get("password").and_then(|v| v.as_str()) {
        inner.password = v.to_string();
    }
    if let Some(v) = parsed.get("access_token").and_then(|v| v.as_str()) {
        inner.access_token = v.to_string();
    }
    if let Some(v) = parsed.get("refresh_token").and_then(|v| v.as_str()) {
        inner.refresh_token = v.to_string();
    }
}
