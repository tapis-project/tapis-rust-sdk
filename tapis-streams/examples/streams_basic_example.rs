use tapis_streams::TapisStreams;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let jwt_token =
        std::env::var("TAPIS_TOKEN").expect("TAPIS_TOKEN environment variable must be set");
    let base_url = std::env::var("TAPIS_BASE_URL")
        .unwrap_or_else(|_| "https://dev.develop.tapis.io/v3".to_string());

    let client = TapisStreams::new(&base_url, &jwt_token)?;
    println!(
        "Initialized TapisStreams against {}",
        client.config().base_path
    );

    Ok(())
}
