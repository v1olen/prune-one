use rocket::http::Status;
use rocket::response::status::*;
use rocket_contrib::json::Json;

use crate::errors::*;
use crate::responses::*;
use crate::establish_connection;
use crate::models::*;
use crate::responses::bridges::*;
use rocket::request::Form;

#[post("/bridge", data = "<new_bridge>")]
pub fn post(new_bridge: Form<NewBridge>) -> Result<ValidResponse<CreatedBridge>, ErrorResponse> {
    use crate::diesel::RunQueryDsl;
    use crate::schema::bridges;

    let connection = establish_connection();

    let slug = {
        let bridge = match diesel::insert_into(bridges::table)
            .values(&new_bridge.into_inner())
            .get_result::<Bridge>(&connection) {
                Ok(data) => data,
                _ => return Err(ERROR_INSERTING_BRIDGE),
            };
        bridge.slug
    };

    Ok(created_bridge!(slug))
}
