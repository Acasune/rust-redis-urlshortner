use serde::Serialize;

#[derive(Serialize)]
pub struct KeyValue {
    pub(crate) hashed: String,
    pub(crate) url: String,
}
