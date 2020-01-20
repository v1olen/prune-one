pub mod bridges;

use crate::establish_connection;
use crate::models::*;
use diesel::prelude::*;
use rocket::response::Redirect;
use std::env;
use std::path::{Path, PathBuf};

#[get("/<query>")]
pub fn index(query: String) -> Result<Redirect, String> {
    use crate::schema::bridges::dsl::*;

    let connection = establish_connection();

    let url = {
        let query = match bridges
            .filter(slug.eq(&query))
            .limit(1)
            .load::<Bridge>(&connection)
        {
            Ok(data) => data,
            _ => return Err(String::from("Cannot load bridges")),
        };

        let bridge = match query.iter().nth(0) {
            Some(data) => data,
            _ => return Err(String::from("Bridge not found")),
        };

        bridge.target_url.clone()
    };

    Ok(Redirect::to(url))
}

use rocket::response::NamedFile;

#[get("/static/<file..>")]
pub fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(
        Path::new(format!(
            "{}/dist/",
            env::var("FRONTEND_PATH").expect("FRONTEND_PATH must be set")
        ).as_str())
        .join(file),
    )
    .ok()
}

#[get("/")]
pub fn home() -> Option<NamedFile> {
    NamedFile::open(Path::new(format!(
        "{}/dist/index.html",
        env::var("FRONTEND_PATH").expect("FRONTEND_PATH must be set")
    ).as_str()))
    .ok()
}

