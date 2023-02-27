#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use utils::contants;

mod command;
mod configs;
mod error;
mod logger;
mod models;
mod schema;
mod services;
mod utils;

fn main() {
    dotenvy::dotenv().ok();
    logger::setup_logger().expect(contants::INIT_LOG_FAIL);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            command::set_cookie,
            command::get_cookie,
            command::logger
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
