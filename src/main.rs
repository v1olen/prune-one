#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use std::env;

#[macro_use]
pub mod responses;
pub mod errors;
pub mod models;
pub mod routes;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

embed_migrations!("migrations/");

fn main() {
    dotenv().ok();
    let args: Vec<String> = env::args().collect();
    match args.iter().nth(1) {
        Some(arg) => {
            if arg == &String::from("types") {
                generate_types().expect("Cannot generate_types");
                return;
            }
        }
        _ => (),
    }
    let connection = establish_connection();

    embedded_migrations::run_with_output(&connection, &mut std::io::stdout()).unwrap();
    rocket::ignite()
        .mount(
            "/",
            routes![
                routes::index,
                routes::files,
                routes::home,
                routes::bridges::post
            ],
        )
        .launch();
}

fn generate_types() -> Result<(), String> {
    if cfg!(any(debug_assertions, feature = "export-typescript")) {
        let shared_models = [
            models::NewBridge::type_script_ify(),
            models::Bridge::type_script_ify(),
            responses::bridges::CreatedBridge::type_script_ify(),
        ];
        use std::fs::File;
        use std::io::prelude::*;
        use typescript_definitions::TypeScriptifyTrait;

        let mut file = File::create(format!(
            "{}/src/types.ts",
            env::var("FRONTEND_PATH").expect("FRONTEND_PATH must be set")
        ))
        .expect("Error creating file");

        for model in shared_models.iter()
        {
            file.write_all(format!("{}\n", model).as_bytes())
                .expect("Error writing to file");
        }
    } else {
        return Err(String::from(""));
    }

    Ok(())
}
