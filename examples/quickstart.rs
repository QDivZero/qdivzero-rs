//! Quick start: list the user's accounts with an API key.
//! Usage: QDIV0_API_KEY=... cargo run --example quickstart

use qdivzero::auth;

#[tokio::main]
async fn main() {
    let api_key = std::env::var("QDIV0_API_KEY").unwrap_or_default();
    if api_key.is_empty() {
        eprintln!("set QDIV0_API_KEY");
        std::process::exit(2);
    }
    auth::set_api_key(api_key);
    auth::load_credentials();

    let client = qdivzero::Client::new("https://api.qdiv0.com");
    match client.get_accounts().await {
        Ok(resp) => {
            for m in resp.into_inner().memberships {
                println!("account={:?} role={:?}", m.account_id, m.role);
            }
        }
        Err(e) => {
            eprintln!("get accounts: {e}");
            std::process::exit(1);
        }
    }
}
