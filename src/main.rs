#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use self::diesel::prelude::*;
#[macro_use] extern crate rocket;

use dotenv::dotenv;
use std::env;

#[macro_use] pub mod responses;
pub mod errors;
pub mod models;
pub mod routes;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    rocket::ignite().mount("/", routes![routes::bridges::post]).launch();
}
