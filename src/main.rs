mod server;

#[tokio::main]
async fn main() {
    // Ensure user root directory exists
    pza_toolkit::path::ensure_user_root_dir_exists()
        .unwrap_or_else(|err| panic!("Failed to ensure user root directory exists: {}", err));

    // Update manifest information
    pza_toolkit::manifest::update_manifest("pza-power-supply");

    // Run the power supply server
    server::run_server().await;
}
