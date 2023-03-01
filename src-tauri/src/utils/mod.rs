pub mod contants;
pub mod response;

#[cfg(debug_assertions)]
pub const ASSETS_DIR: &str = "assets-dev";
#[cfg(not(debug_assertions))]
pub const ASSETS_DIR: &str = "assets";
