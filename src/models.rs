#[derive(Queryable)]
pub struct Bridge {
    pub id: i32,
    pub slug: String,
    pub target_url: String,
    pub active: bool,
}
