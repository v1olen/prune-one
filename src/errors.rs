use super::responses::*;

use rocket::http::Status;
use rocket::response::status::*;
use rocket_contrib::json::Json;

macro_rules! new_error {
    ($status:expr => $message:expr) => {
        Custom($status, Json(ErrorMessage { message: $message }))
    };
}

#[allow(dead_code)]
pub const UNIMPLEMENTED: ErrorResponse = new_error!(
    Status::InternalServerError => "Internal server error"
);

pub const ERROR_INSERTING_BRIDGE: ErrorResponse = new_error!(
    Status::InternalServerError => "Internal server error"
);

pub const NO_PROTOCOL_IN_TARGET: ErrorResponse = new_error!(
    Status::BadRequest => "Protocol must be specified in target"
);
