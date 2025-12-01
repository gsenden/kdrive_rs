use kdrive_service::kdrive::kdrive_service_client::KdriveServiceClient;
use kdrive_service::kdrive::Empty;
use std::io;
use tonic::Request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Connecting to gRPC server...");

    let channel = tonic::transport::Channel::from_static("http://127.0.0.1:50051")
        .connect()
        .await?;

    let mut client = KdriveServiceClient::new(channel);

    // Step 1: Check if already authenticated
    println!("\n[1] Checking authentication status...");
    let resp = client.is_authenticated(Request::new(Empty {})).await?;
    println!("Is authenticated: {}", resp.into_inner().is_authenticated);

    // Step 2: Start auth flow
    println!("\n[2] Starting auth flow...");
    let resp = client.start_initial_auth_flow(Request::new(Empty {})).await?;
    let auth_url = resp.into_inner().auth_url;
    println!("Auth URL: {}", auth_url);
    println!("\nPlease open this URL in your browser and log in.");
    println!("Press Enter when you're done...");

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    // Step 3: Complete auth flow
    println!("\n[3] Completing auth flow...");
    let resp = client.continue_initial_auth_flow(Request::new(Empty {})).await?;
    let success = resp.into_inner().success;
    println!("Auth flow success: {}", success);

    // Step 4: Check if now authenticated
    println!("\n[4] Checking authentication status again...");
    let resp = client.is_authenticated(Request::new(Empty {})).await?;
    println!("Is authenticated: {}", resp.into_inner().is_authenticated);

    Ok(())
}