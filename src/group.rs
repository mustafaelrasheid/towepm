use std::collections::HashSet;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Group {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub atoms: HashSet<String>,
}

impl Group {
    pub fn new(name: &str) -> Self {
        return Self {
            name: name.to_string(),
            description: None,
            atoms: HashSet::new(),
        }
    }
}
