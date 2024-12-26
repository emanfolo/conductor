use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum StatusCode {
    Ok,
    BadRequest,
    InternalServerError,
    NotFound,
}

pub enum ApiError {
    TaskCreationFailed(String),
    InvalidInput(String)
}