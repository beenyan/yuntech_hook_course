use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Response<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

impl<T> Response<T> {
    pub fn new(success: bool, message: &str, data: Option<T>) -> Response<T> {
        Response {
            success,
            message: message.to_string(),
            data,
        }
    }

    pub fn success(message: &str, data: Option<T>) -> Response<T> {
        Response {
            success: true,
            message: message.to_string(),
            data,
        }
    }

    pub fn fail(message: &str, data: Option<T>) -> Response<T> {
        Response {
            success: false,
            message: message.to_string(),
            data,
        }
    }
}
