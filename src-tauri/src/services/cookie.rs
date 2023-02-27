use crate::{
    configs::db::connect_pool,
    models::cookies::{DieselCookie, DieselCookieDto},
};
use log::error;

pub fn set_cookie(cookie_list: &Vec<String>) -> Result<usize, String> {
    let cookie_list = cookie_list
        .iter()
        .filter_map(|c| DieselCookieDto::new(&c).ok())
        .collect::<Vec<DieselCookieDto>>();

    match DieselCookie::insert(&cookie_list, &mut connect_pool().get().unwrap()) {
        Ok(rows_inserted) => Ok(rows_inserted),
        Err(err) => {
            let err_msg = format!("新增 Cookie 失敗: {err}");
            error!("{err_msg}");
            Err(err_msg)
        }
    }
}

pub fn get_cookie() -> Result<String, String> {
    let cookie_list = match DieselCookie::find_all(&mut connect_pool().get().unwrap()) {
        Ok(data) => data,
        Err(err) => {
            let err_msg = format!("讀取 Cookie 失敗: {err}");
            error!("{err_msg}");
            return Err(err_msg);
        }
    };

    let cookie_str = cookie_list
        .iter()
        .map(|c| format!("{}={}", c.name, c.value))
        .collect::<Vec<String>>()
        .join("; ");

    Ok(cookie_str)
}
