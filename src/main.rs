mod constants;
mod path;
mod server;

#[tokio::main]
async fn main() {
    // Update manifest information
    pza_toolkit::manifest::update_manifest("pza-power-supply");

    // Run the power supply server
    server::run_server().await;
}
