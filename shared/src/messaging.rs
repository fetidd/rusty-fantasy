use serde::{Deserialize, Serialize};

use crate::modifier::ModifierMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum SystemResponse {
    Chat {username: String, role: String, content: String},
    Roll {dice_values: (i8, i8), username: String, modifiers: ModifierMap, total: i8},
    SceneUpdate {},
    HeroUpdate {},
    ChallengeUpdate {},
    FellowshipUpdate {},
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum SystemRequest {
    Chat {username: String, role: String, content: String},
    Roll {username: String, modifiers: ModifierMap},
    SceneUpdate {},
    HeroUpdate {},
    ChallengeUpdate {},
    FellowshipUpdate {},
}

