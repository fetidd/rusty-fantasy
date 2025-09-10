use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)  ]
pub enum Might {
    Origin,
    Adventure,
    Greatness
}