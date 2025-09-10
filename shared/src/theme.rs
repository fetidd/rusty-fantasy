use serde::{Deserialize, Serialize};

use crate::{might::Might, tag::Tag};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Theme {
    pub name: String,
    pub power_tags: Vec<Tag>,
    pub weakness_tags: Vec<Tag>,
    pub might: Might,
    pub quest: String,
    pub improve: u8,
    pub abandon: u8,
    pub milestone: u8,
    pub special_improvements: Vec<String>,
}

impl Theme {
    pub fn get_value(&self) -> u8 {
        1
    }
}
