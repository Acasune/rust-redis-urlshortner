use serde::Serialize;
#[derive(Serialize)]
pub struct ResponseBodyInit {
    pub(crate) urls: Vec<String>,
}
#[derive(Serialize)]
pub struct ResponseBodyGetUrl {
    pub(crate) url: String,
}
#[derive(Serialize)]
pub struct ResponseBodyPostUrl {
    pub(crate) hashed_url: String,
}
#[derive(Serialize)]
pub struct ResponseBodyDeleteUrl {
    pub(crate) hashed_url: String,
}
