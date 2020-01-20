#[macro_use] use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct CreatedBridge {
    pub slug: String,
}

#[macro_export]
macro_rules! created_bridge {
    ($slug:expr) => {
        Custom(Status::Created, Json(CreatedBridge { slug: $slug }))
    };
}
