<p align="center">
  <img src="assets/qdiv0-mark.png" alt="QDivZero" width="256">
</p>

# qdivzero-rs

Rust client for the [QDivZero API](https://api.qdiv0.com), generated from its
[OpenAPI specification](https://api.qdiv0.com/openapi).

[![CI](https://github.com/QDivZero/qdivzero-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/QDivZero/qdivzero-rs/actions/workflows/ci.yml)

## Install

Not published on crates.io yet — depend on the git repository (pin a release
tag):

````sh
cargo add qdivzero-rs --git https://github.com/QDivZero/qdivzero-rs.git --tag v1.0.0
````

## Quick start

Token-first: set an API key (or load credentials), then call the client.

````rust
use qdivzero::auth;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    auth::set_api_key("your-api-key");
    auth::load_credentials();

    let client = qdivzero::Client::new("https://api.qdiv0.com");
    let resp = client.get_accounts().await?;

    for m in resp.into_inner().memberships {
        println!("account={:?} role={:?}", m.account_id, m.role);
    }
    Ok(())
}
````

## Authentication

Token-based authentication is the primary way to use the client. State is
global and process-wide; set it once at startup:

- `auth::set_api_key(key)` authenticates with a QDivZero API key
  (`Authorization: Bearer <key>`).
- `auth::set_access_token(token, expiry_unix)` / `auth::set_refresh_token(token)`
  pre-configure bearer tokens without calling login.
- `auth::login(&client, email, password)` authenticates with credentials and
  stores the access and refresh tokens.
- `auth::refresh(&client)` forces a token refresh (refresh token first, then
  stored credentials).
- `auth::retry_after_refresh(&client, || client.get_auth_me())` runs the
  closure; on a `401` error response it refreshes the token once and retries.
- For proactive refresh, check `auth::token_near_expiry()` (true when the
  access token has less than 30 seconds of life left); the stored expiry is
  available as `auth::STATE.lock().unwrap().access_token_expiry`.

A pre-hook injects `Authorization` automatically on every request — there is
nothing to pass per call.

## Credentials file

If `~/.qdivzero/credentials` exists, `auth::load_credentials()` applies it to
the auth state. All fields are optional:

````json
{
  "email": "you@example.com",
  "password": "your-password",
  "access_token": "...",
  "refresh_token": "...",
  "api_key": "..."
}
````

`email`/`password` enable on-demand login and fallback refresh;
`access_token`/`refresh_token` authenticate directly. Keep the file private
(`chmod 600 ~/.qdivzero/credentials`) and never commit it.

## Error handling

Operations return `Result<ResponseValue<T>, Error<ErrorResponse>>`. Success
responses carry `resp.status()` and the typed payload via `resp.into_inner()`;
server/API errors arrive as `Error::ErrorResponse`, transport-level failures as
other `Error` variants:

````rust
match client.get_accounts().await {
    Ok(resp) => { /* resp.status(), resp.into_inner() */ }
    Err(e) => eprintln!("error: {e}"),
}
````

## Regeneration

The library is regenerated automatically from
`https://api.qdiv0.com/openapi` by the update workflow (daily at 06:00 UTC, or
manually via *Actions → update → Run workflow*). The upstream spec snapshot is
committed at `api/openapi.json`; the pipeline preprocesses it for progenitor
and commits the result at `api/openapi.rs.json`.

**`src/lib_gen.rs` is generated — do not edit it by hand.** A new release tag
(`v1.0.<n>`) is published on every change.

Known limitation: the two multipart endpoints (`/v1/audio/transcriptions`,
`/v1/images/edits`) are not generated, because progenitor has no multipart
support.

## Development

Regenerate locally:

````sh
scripts/preprocess-rs.sh && cargo build
````

Checks: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`,
`cargo test`.
