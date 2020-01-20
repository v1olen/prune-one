#[macro_use] pub mod bridges;

use rocket::response::status::*;
use rocket_contrib::json::Json;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct ErrorMessage {
    pub message: &'static str,
}

pub type ErrorResponse = Custom<Json<ErrorMessage>>;
pub type ValidResponse<T> = Custom<Json<T>>;
