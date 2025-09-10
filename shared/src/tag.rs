use std::collections::HashSet;

use serde::{Deserialize, Serialize};

/// A tag represents a characteristic, trait, or condition that can affect a character's abilities in the game.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Tag {
    Power { name: String, is_scratched: bool },
    Weakness { name: String },
    Story { name: String, is_scratched: bool },
    Status { name: String, tiers: HashSet<u8> },
}

impl Tag {
    /// Get the value of the tag for roll calculations.
    pub fn get_value(&self) -> u8 {
        match self {
            Tag::Power { is_scratched, .. } | Tag::Story { is_scratched, .. } => {
                if *is_scratched {
                    0
                } else {
                    1
                }
            }
            Tag::Weakness { .. } => 1,
            Tag::Status { tiers, .. } => {
                if let Some(highest) = tiers.iter().max() {
                    *highest
                } else {
                    0
                }
            }
        }
    }

    /// Add a tier to a status tag, up to a maximum of 6.
    pub fn add_tier(&mut self, mut tier: u8) {
        // add a tier to a status tag, up to a maximum of 6
        match self {
            Tag::Status { tiers, .. } => {
                while tiers.contains(&tier) && tier < 6 {
                    tier += 1;
                }
                tiers.insert(tier);
            }
            _ => (),
        }
    }

    /// Decrease the tier of a status tag by 1, removing any that reach 0.
    pub fn decrease_tier(&mut self) {
        // decrease all tiers by 1, removing any that reach 0
        match self {
            Tag::Status { tiers, .. } => {
                *tiers = tiers.iter().filter(|&&t| t > 1).map(|&t| t - 1).collect();
            }
            _ => (),
        }
    }

    /// Scratch a power or story tag.
    pub fn scratch(&mut self) {
        match self {
            Tag::Power { is_scratched, .. } | Tag::Story { is_scratched, .. } => {
                *is_scratched = true;
            }
            _ => (),
        }
    }

    /// Unscratch a power or story tag.
    pub fn unscratch(&mut self) {
        match self {
            Tag::Power { is_scratched, .. } | Tag::Story { is_scratched, .. } => {
                *is_scratched = false;
            }
            _ => (),
        }
    }

    /// Create a new power tag with the given name.
    pub fn new_power(name: &str) -> Self {
        Tag::Power {
            name: name.to_string(),
            is_scratched: false,
        }
    }

    /// Create a new weakness tag with the given name.
    pub fn new_weakness(name: &str) -> Self {
        Tag::Weakness {
            name: name.to_string(),
        }
    }

    /// Create a new story tag with the given name.
    pub fn new_story(name: &str) -> Self {
        Tag::Story {
            name: name.to_string(),
            is_scratched: false,
        }
    }

    /// Create a new status tag with the given name and initial tier.
    pub fn new_status(name: &str, initial_tier: usize) -> Self {
        Tag::Status {
            name: name.to_string(),
            tiers: HashSet::from([initial_tier as u8]),
        }
    }
}

