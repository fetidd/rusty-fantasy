use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum SystemResponse {
    Chat {username: String, role: String, content: String},
    Roll {dice_values: (u8, u8), username: String, tags: TagMap},
    SceneUpdate {},
    HeroUpdate {},
    ChallengeUpdate {},
    FellowshipUpdate {},
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum SystemRequest {
    Chat {username: String, role: String, content: String},
    Roll {dice_values: (u8, u8), username: String, tags: TagMap},
    SceneUpdate {},
    HeroUpdate {},
    ChallengeUpdate {},
    FellowshipUpdate {},
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TagMap {
    tags: std::collections::HashMap<String, u8>,
}