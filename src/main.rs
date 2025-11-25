mod constants;
mod path;
mod server;

#[tokio::main]
async fn main() {
    server::run_server().await;
}
