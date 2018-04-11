use super::*;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Course {
    pub name: String,
    pub sections: Vec<Section>,
    pub active_section: Option<Section>,
}

impl Course {
    pub fn jzero() -> Self {
        Course {
            name: "Jzero".into(),
            sections: vec![
                Section::vocabulary_group_a(),
            ],
            active_section: None,
        }
    }
}
