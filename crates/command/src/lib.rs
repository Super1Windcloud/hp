#![deny(clippy::shadow)]
pub mod buckets;
pub mod cat;
pub mod init_env;
pub mod list;
pub mod merge;
pub mod search;
pub mod utils;
use anyhow;
pub use init_env::HyperScoop;
pub use list::{display_app_info, list_specific_installed_apps};
pub use std::process::exit;
// pub use manifest::search_manifest;
pub mod manifest;

pub fn init_hyperscoop() -> Result<HyperScoop, anyhow::Error> {
    let hyperscoop = init_env::HyperScoop::new();
    Ok(hyperscoop)
}
