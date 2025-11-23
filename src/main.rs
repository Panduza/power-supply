mod config;
mod constants;
mod drivers;
mod path;
mod server;

fn main() {
    server::run_server();
}
