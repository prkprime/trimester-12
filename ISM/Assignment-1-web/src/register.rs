use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct Register {
    pub username: String,
    pub password: String,
}
