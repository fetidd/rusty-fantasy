use serde::{Deserialize, Serialize};

use crate::{tag::Tag, theme::Theme};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Modifier {
    Tag(Tag),
    Theme(Theme),
}

impl Modifier {
    pub fn get_value(&self) -> u8 {
        match self {
            Modifier::Tag(tag) => tag.get_value(),
            Modifier::Theme(theme) => theme.get_value(),
        }
    }
}

/// The polarity of a tag, indicating whether it has a positive or negative effect.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Polarity {
    Positive,
    Negative,
}

/// A collection of tags associated with a character, along with their polarities and whether they are burned.
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct ModifierMap {
    modifiers: std::collections::HashMap<String, (Modifier, Polarity, bool)>,
}

impl ModifierMap {
    /// Add a tag to the TagMap with the specified polarity and burned status.
    pub fn add_tag(&mut self, tag: Tag, polarity: Polarity, is_burned: bool) {
        let name = match &tag {
            Tag::Power { name, .. }
            | Tag::Weakness { name, .. }
            | Tag::Story { name, .. }
            | Tag::Status { name, .. } => name.clone(),
        };
        self.modifiers.insert(name, (Modifier::Tag(tag), polarity, is_burned));
    }

    pub fn add_theme(&mut self, theme: Theme, polarity: Polarity, is_burned: bool) {
        let name = theme.name.to_string();
        self.modifiers.insert(name, (Modifier::Theme(theme), polarity, is_burned));
    }

    pub fn add_burned_tag(&mut self, tag: Tag) {
        self.add_tag(tag, Polarity::Positive, true);
    }

    pub fn add_positive_tag(&mut self, tag: Tag) {
        self.add_tag(tag, Polarity::Positive, false);
    }

    pub fn add_burned_theme(&mut self, theme: Theme) {
        self.add_theme(theme, Polarity::Positive, true);
    }

    pub fn add_positive_theme(&mut self, theme: Theme) {
        self.add_theme(theme, Polarity::Positive, false);
    }

    pub fn add_negative_tag(&mut self, tag: Tag) {
        self.add_tag(tag, Polarity::Negative, false);
    }

    pub fn add_negative_theme(&mut self, theme: Theme) {
        self.add_theme(theme, Polarity::Negative, false);
    }

    pub fn remove_modifier(&mut self, name: &str) {
        self.modifiers.remove(name);
    }

    pub fn get_modifiers(&self) -> Vec<(&Modifier, &Polarity, bool)> {
        self.modifiers
            .values()
            .map(|(modifier, polarity, is_burned)| (modifier, polarity, *is_burned))
            .collect()
    }
}

/// Display the tags in a human-readable format.
impl std::fmt::Display for ModifierMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tags: Vec<String> = self
            .modifiers
            .iter()
            .map(|(name, (modifier, polarity, is_burned))| {
                let sign = match polarity {
                    Polarity::Positive => "+",
                    Polarity::Negative => "-",
                };
                match modifier {
                    Modifier::Tag(tag) => match tag {
                        Tag::Power { .. } | Tag::Story { .. } => {
                            if *is_burned {
                                format!("{sign}{sign}{sign}[{}]{sign}{sign}{sign}", name)
                            } else {
                                format!("{sign}[{}]{sign}", name)
                            }
                        }
                        Tag::Weakness { .. } => {
                            format!("{sign}[{}]{sign}", name)
                        }
                        Tag::Status { name, tiers } => {
                            let mut tiers: Vec<u8> = tiers.iter().cloned().collect();
                            tiers.sort();
                            let tier_str = tiers
                                .iter()
                                .map(|t| t.to_string())
                                .collect::<Vec<String>>()
                                .join(",");
                            format!("{sign}{{{}:{}}}{sign}", name, tier_str)
                        }
                    },
                    Modifier::Theme(theme) => todo!(),
                }
            })
            .collect();
        write!(f, "{}", tags.join(" "))
    }
}
