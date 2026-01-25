/// Example demonstrating the TapisAuthenticator client wrapper
///
/// This example shows how to use the high-level wrapper to interact
/// with the Tapis Authenticator API, including both authenticated and
/// unauthenticated endpoints.
///
/// # Setup
///
/// For authenticated examples, set your JWT token as an environment variable:
/// ```bash
/// export TAPIS_TOKEN="your_jwt_token_here"
/// ```
///
/// For token creation, set your username and password:
/// ```bash
/// export TAPIS_USERNAME="your_username"
/// export TAPIS_PASSWORD="your_password"
/// ```
///
/// # Run
///
/// ```bash
/// cargo run --example authenticator_example
/// ```

use tapis_authenticator::client::TapisAuthenticator;
use tapis_authenticator::models::NewToken;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔐 Tapis Authenticator SDK Examples");
    println!("════════════════════════════════════════\n");

    // Example 0: Create JWT Token (Unauthenticated)
    println!("📋 Example 0: Create JWT Token (No Auth Required)");
    println!("───────────────────────────────────────────────────");
    
    // Get TAPIS_HOST from environment or use default
    let tapis_host = std::env::var("TAPIS_HOST").unwrap_or_else(|_| "tacc.tapis.io".to_string());

    // Note: base_url should NOT include the service path (/authenticator) or /v3
    // because the generated API code already includes the full path (e.g., /v3/oauth2/tokens)
    let base_url = format!("https://{}", tapis_host);
    
    println!("🌍 Using Tapis host: {}", tapis_host);
    println!("📡 Base URL: {}\n", base_url);

    // This endpoint doesn't require authentication
    let public_client = TapisAuthenticator::new(
        &base_url,
        None, // No JWT token needed for token creation
    )?;
    
    // Try to get username/password from environment
    if let (Ok(username), Ok(password)) = (
        std::env::var("TAPIS_USERNAME"),
        std::env::var("TAPIS_PASSWORD"),
    ) {
        println!("✓ Creating JWT token for user: {}", username);
        
        let new_token = NewToken {
            username: Some(username),
            password: Some(password),
            grant_type: Some("password".to_string()),
            ..Default::default()
        };
        
        match public_client.tokens.create_token(new_token).await {
            Ok(response) => {
                println!("✓ JWT token created successfully!");
                println!("  Status: {:?}", response.status);
                if let Some(result) = response.result {
                    if let Some(access_token_str) = result.access_token.access_token {
                        println!("  Access Token: {}", access_token_str);
                        // set TAPIS_TOKEN environment variable for subsequent examples
                        std::env::set_var("TAPIS_TOKEN", access_token_str);
                    }
                    if let Some(expires_in) = result.access_token.expires_in {
                        println!("  Expires In: {} seconds", expires_in);
                    }
                }
            }
            Err(e) => eprintln!("✗ Failed to create token: {:?}", e),
        }
    } else {
        println!("ℹ️  Skipping token creation - set TAPIS_USERNAME and TAPIS_PASSWORD to test");
        println!("   Example: export TAPIS_USERNAME=myuser TAPIS_PASSWORD=mypass");
    }
    println!();

    // Get JWT token from environment for authenticated examples
    let jwt_token = match std::env::var("TAPIS_TOKEN") {
        Ok(token) => token,
        Err(_) => {
            println!("ℹ️  TAPIS_TOKEN not set. Skipping authenticated examples.");
            println!("   Set it with: export TAPIS_TOKEN='your_token'");
            println!("\n✅ Example completed (partial - no auth token)");
            return Ok(());
        }
    };

    println!("🔧 Creating authenticated TapisAuthenticator client...");
    
    // Create the high-level client with authentication
    let client = TapisAuthenticator::new(
        &base_url,
        Some(&jwt_token),
    )?;

    println!("✅ Client created successfully!\n");

    // Example 1: Health Check
    println!("📋 Example 1: Health Check");
    println!("──────────────────────────");
    match client.health_check.hello().await {
        Ok(response) => {
            println!("✓ Health check passed");
            println!("  Message: {:?}", response.message);
        }
        Err(e) => eprintln!("✗ Health check failed: {:?}", e),
    }
    println!();

    // Example 2: Get Server Metadata
    println!("📋 Example 2: Server Metadata");
    println!("──────────────────────────────");
    match client.metadata.get_server_metadata().await {
        Ok(response) => {
            println!("✓ Server metadata retrieved");
            println!("  Status: {:?}", response.status);
            println!("  Version: {:?}", response.version);
        }
        Err(e) => eprintln!("✗ Failed to get metadata: {:?}", e),
    }
    println!();

    // Example 3: Get User Information
    println!("📋 Example 3: User Information");
    println!("───────────────────────────────");
    match client.profiles.get_userinfo().await {
        Ok(response) => {
            println!("✓ User information retrieved");
            println!("  Status: {:?}", response.status);
            if let Some(result) = response.result {
                println!("  User data: {:#?}", result);
            }
        }
        Err(e) => eprintln!("✗ Failed to get user info: {:?}", e),
    }
    println!();

    // Example 4: List OAuth2 Clients
    println!("📋 Example 4: List OAuth2 Clients");
    println!("───────────────────────────────────");
    match client.clients.list_clients(Some(10), Some(0)).await {
        Ok(response) => {
            println!("✓ OAuth2 clients retrieved");
            println!("  Status: {:?}", response.status);
            if let Some(result) = response.result {
                println!("  Number of clients: {}", result.len());
                for (i, client_info) in result.iter().enumerate().take(3) {
                    println!("  Client {}: {:#?}", i + 1, client_info);
                }
            }
        }
        Err(e) => eprintln!("✗ Failed to list clients: {:?}", e),
    }
    println!();

    println!("✅ All examples completed!");
    println!();
    println!("📚 Available API Modules:");
    println!("  • client.admin       - Tenant configuration management");
    println!("  • client.clients     - OAuth2 client management");
    println!("  • client.health_check - Health and readiness checks");
    println!("  • client.metadata    - Server metadata and version info");
    println!("  • client.profiles    - User profile management");
    println!("  • client.tokens      - Token creation and management");

    Ok(())
}
