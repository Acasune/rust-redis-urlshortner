use serde::Serialize;

use crate::types::KeyValue;
#[derive(Serialize)]
pub struct ResponseBodyInit {
    pub(crate) urls: Vec<KeyValue>,
}
#[derive(Serialize)]
pub struct ResponseBodyGetUrl {
    pub(crate) url: String,
}
#[derive(Serialize)]
pub struct ResponseBodyPostUrl {
    pub(crate) hashed: String,
}
#[derive(Serialize)]
pub struct ResponseBodyDeleteUrl {
    pub(crate) hashed: String,
}
