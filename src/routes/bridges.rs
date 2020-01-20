use rocket::http::Status;
use rocket::response::status::*;
use rocket_contrib::json::Json;

use crate::errors::*;
use crate::responses::*;
use crate::establish_connection;
use crate::models::*;
use crate::responses::bridges::*;

#[post("/bridge", data = "<new_bridge>")]
pub fn post(new_bridge: Json<NewBridge>) -> Result<ValidResponse<CreatedBridge>, ErrorResponse> {
    use crate::diesel::RunQueryDsl;
    use crate::schema::bridges;

    let connection = establish_connection();

    let new_bridge = new_bridge.into_inner();
    let pattern = regex::Regex::new("^https?://").unwrap();

    match pattern.captures(new_bridge.target_url.as_str()) {
        None => return Err(NO_PROTOCOL_IN_TARGET),
        _ => (),
    }

    let slug = {
        let bridge = match diesel::insert_into(bridges::table)
            .values(&new_bridge)
            .get_result::<Bridge>(&connection) {
                Ok(data) => data,
                _ => return Err(ERROR_INSERTING_BRIDGE),
            };
        bridge.slug
    };

    let url = format!("{domain}/{slug}", domain=std::env::var("PRUNING_DOMAIN").unwrap(), slug=slug);

    Ok(created_bridge!(slug, url))
}
