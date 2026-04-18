pub mod config;
pub mod display;

pub use config::{Config, ConfigError, Profile, Resolution};
pub use display::{DisplayError, get_current_resolution, set_resolution};
