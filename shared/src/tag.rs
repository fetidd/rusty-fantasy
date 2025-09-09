#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum Tag {
    Power { name: String, is_scratched: bool },
    Weakness { name: String, is_scratched: bool },
}
impl Tag {
    pub fn new_power(name: &str) -> Self {
        Tag::Power { name: name.to_string(), is_scratched: false }
    }

    pub fn new_weakness(name: &str) -> Self {
        Tag::Weakness { name: name.to_string(), is_scratched: false }
    }
}