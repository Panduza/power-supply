mod error;
mod power_state;

pub use error::ErrorPayload;
pub use power_state::{PowerState, PowerStatePayload};

/// Generate a random 5-character PZA ID
pub fn generate_pza_id() -> String {
    pza_toolkit::rand::generate_random_string(5)
}
