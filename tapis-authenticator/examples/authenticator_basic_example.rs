use tapis_authenticator::TapisAuthenticator;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = std::env::var("TAPIS_BASE_URL")
        .unwrap_or_else(|_| "https://dev.develop.tapis.io/v3".to_string());

    let client = TapisAuthenticator::new(&base_url, None)?;
    println!(
        "Initialized TapisAuthenticator against {}",
        client.config().base_path
    );

    Ok(())
}
