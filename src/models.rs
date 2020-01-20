use super::schema::bridges;
use serde::{Serialize, Deserialize};
use typescript_definitions::TypeScriptify;

#[derive(Insertable, TypeScriptify, Serialize, Deserialize, Debug)]
#[table_name = "bridges"]
pub struct NewBridge {
    pub slug: String,
    pub target_url: String,
}

#[derive(Queryable, TypeScriptify, Serialize)]
pub struct Bridge {
    pub slug: String,
    pub target_url: String,
    pub active: bool,
}
