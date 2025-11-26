mod error;
mod power_state;
mod status;

pub use error::ErrorPayload;
pub use power_state::{PowerState, PowerStatePayload};
pub use status::StatusPayload;

/// Generate a random 5-character PZA ID
pub fn generate_pza_id() -> String {
    pza_toolkit::rand::generate_random_string(5)
}
