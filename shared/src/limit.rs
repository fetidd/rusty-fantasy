use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Limit {
    pub name: String,
    pub progress: u8,
    pub max: u8,
}