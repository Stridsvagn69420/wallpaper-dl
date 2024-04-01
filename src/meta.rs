//! Crate Metadata
//! 
//! Submodule for metadata about this crate.

// App Metadata
pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const APP_DESC: &str = env!("CARGO_PKG_DESCRIPTION");

// Config Constants
pub const CONFIG_FILE: &str = "config.toml";

// HTTP Constants
pub const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), '/', env!("CARGO_PKG_VERSION")); 