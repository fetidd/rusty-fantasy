use serde::{Deserialize, Serialize};

use crate::{theme::Theme};

use super::{tag::Tag};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Hero {
    pub name: String,
    pub player: String,
    pub themes: Vec<Theme>,
    pub backpack: Vec<Tag>,
}

