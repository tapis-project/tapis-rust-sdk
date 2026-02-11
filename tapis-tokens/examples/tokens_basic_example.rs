use tapis_tokens::TapisTokens;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let jwt_token = std::env::var("TAPIS_TOKEN").ok();
    let base_url = std::env::var("TAPIS_BASE_URL")
        .unwrap_or_else(|_| "https://dev.develop.tapis.io/v3".to_string());

    let client = TapisTokens::new(&base_url, jwt_token.as_deref())?;
    println!(
        "Initialized TapisTokens against {}",
        client.config().base_path
    );

    Ok(())
}
