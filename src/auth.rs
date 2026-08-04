//! Authentication state for the QDivZero API client (hand-written, not generated).

use std::sync::Mutex;

pub struct AuthInner {
    pub api_key: String,
    pub access_token: String,
    pub refresh_token: String,
    pub email: String,
    pub password: String,
}

pub static STATE: Mutex<AuthInner> = Mutex::new(AuthInner {
    api_key: String::new(),
    access_token: String::new(),
    refresh_token: String::new(),
    email: String::new(),
    password: String::new(),
});

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
    }
    Ok(())
}
