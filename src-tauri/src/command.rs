use crate::{
    services::cookie,
    utils::{contants, response::Response},
};
use log::info;
use std::usize;

#[tauri::command]
pub fn set_cookie(cookie_list: Vec<String>) -> Response<usize> {
    match cookie::set_cookie(&cookie_list) {
        Ok(rows_inserted) => {
            Response::success(contants::INSERT_COOKIE_SUCCESS, Some(rows_inserted))
        }
        Err(err) => Response::fail(&err, None),
    }
}

#[tauri::command]
pub async fn get_cookie() -> Response<String> {
    match cookie::get_cookie() {
        Ok(cookie_str) => Response::success(contants::GET_COOKIE_SUCCESS, Some(cookie_str)),
        Err(err) => Response::fail(&err, None),
    }
}

#[tauri::command]
pub async fn logger(message: String) {
    info!("{message}");
}
