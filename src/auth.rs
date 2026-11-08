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
    let mut cookies = Vec::new();
    if !inner.access_token.is_empty() {
        cookies.push(format!("qdiv0_access={}", inner.access_token));
    } else if !inner.api_key.is_empty()
        && !request
            .headers()
            .contains_key(reqwest::header::AUTHORIZATION)
    {
        let value = format!("Bearer {}", inner.api_key);
        let header = reqwest::header::HeaderValue::from_str(&value)
            .map_err(|_| "invalid Authorization header value".to_string())?;
        request
            .headers_mut()
            .insert(reqwest::header::AUTHORIZATION, header);
    }
    if request.url().path().ends_with("/auth/refresh") && !inner.refresh_token.is_empty() {
        cookies.push(format!("qdiv0_refresh={}", inner.refresh_token));
    }
    if !cookies.is_empty() {
        if let Some(existing) = request.headers().get(reqwest::header::COOKIE) {
            let existing = existing
                .to_str()
                .map_err(|_| "invalid Cookie header".to_string())?;
            for part in existing
                .split(';')
                .map(str::trim)
                .filter(|part| !part.is_empty())
            {
                let name = part
                    .split_once('=')
                    .map(|(name, _)| name.trim())
                    .unwrap_or("");
                if (name == "qdiv0_access" && !inner.access_token.is_empty())
                    || (name == "qdiv0_refresh"
                        && request.url().path().ends_with("/auth/refresh")
                        && !inner.refresh_token.is_empty())
                {
                    continue;
                }
                cookies.push(part.to_string());
            }
        }
        let header = reqwest::header::HeaderValue::from_str(&cookies.join("; "))
            .map_err(|_| "invalid session cookie".to_string())?;
        request
            .headers_mut()
            .insert(reqwest::header::COOKIE, header);
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

/// Returns the current API key.
pub fn api_key() -> String {
    STATE.lock().unwrap().api_key.clone()
}

/// Authenticates with email/password and stores the returned tokens.
#[allow(clippy::result_large_err)] // Keep the generated client's public error type.
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
    if let Err(err) = store_session(resp.headers()) {
        if let Some(step) = resp.into_inner().next_step.filter(|step| !step.is_empty()) {
            return Err(Error::Custom(format!(
                "login: verification required: {step}"
            )));
        }
        return Err(Error::Custom(format!("login: {err}")));
    }
    Ok(())
}

/// Refreshes the access token (refresh token first, then credentials).
#[allow(clippy::result_large_err)] // Keep the generated client's public error type.
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
        let resp = client.post_auth_refresh().await?;
        store_session(resp.headers()).map_err(|err| Error::Custom(format!("refresh: {err}")))?;
        return Ok(());
    }
    if !email.is_empty() && !password.is_empty() {
        return login(client, &email, &password).await;
    }
    Err(Error::Custom(
        "no refresh token or credentials available".to_string(),
    ))
}

fn store_session(headers: &reqwest::header::HeaderMap) -> Result<(), String> {
    let mut access = None;
    let mut refresh = None;
    let mut expiry = 0;
    for value in headers.get_all(reqwest::header::SET_COOKIE) {
        let Ok(value) = value.to_str() else { continue };
        let mut parts = value.split(';');
        let Some((name, token)) = parts.next().and_then(|part| part.trim().split_once('=')) else {
            continue;
        };
        let mut max_age = None;
        for part in parts {
            if let Some((key, value)) = part.trim().split_once('=') {
                if key.eq_ignore_ascii_case("max-age") {
                    max_age = value.parse::<i64>().ok();
                }
            }
        }
        if token.is_empty() || max_age.is_some_and(|age| age <= 0) {
            continue;
        }
        match name {
            "qdiv0_access" => {
                access = Some(token.to_string());
                if let Some(age) = max_age {
                    expiry = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .map(|now| now.as_secs().saturating_add(age as u64))
                        .unwrap_or(0);
                }
            }
            "qdiv0_refresh" => refresh = Some(token.to_string()),
            _ => {}
        }
    }
    let access = access.ok_or_else(|| "missing access session cookie".to_string())?;
    let refresh = refresh.ok_or_else(|| "missing refresh session cookie".to_string())?;
    let mut inner = STATE.lock().unwrap();
    inner.access_token = access;
    inner.refresh_token = refresh;
    inner.access_token_expiry.store(expiry, Ordering::SeqCst);
    Ok(())
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
#[allow(clippy::result_large_err)] // Keep the generated client's public error type.
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
    if let Some(v) = parsed.get("api_key").and_then(|v| v.as_str()) {
        inner.api_key = v.to_string();
    }
}
