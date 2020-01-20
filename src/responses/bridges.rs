#[macro_use]use serde::Serialize;
use typescript_definitions::TypeScriptify;

#[derive(Serialize, Debug, TypeScriptify)]
pub struct CreatedBridge {
    pub slug: String,
    pub url: String,
}

#[macro_export]
macro_rules! created_bridge {
    ($slug:expr, $url:expr) => {
        Custom(Status::Created, Json(CreatedBridge { slug: $slug, url: $url }))
    };
}
