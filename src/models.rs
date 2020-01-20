use super::schema::bridges;

#[derive(Insertable, FromForm)]
#[table_name = "bridges"]
pub struct NewBridge {
    pub slug: String,
    pub target_url: String,
}

#[derive(Queryable)]
pub struct Bridge {
    pub slug: String,
    pub target_url: String,
    pub active: bool,
}
