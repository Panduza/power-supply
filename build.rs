fn main() {
    // Get the version from Cargo.toml and make it available at compile time
    let version = env!("CARGO_PKG_VERSION");
    println!("cargo:rustc-env=PKG_VERSION={}", version);
}
