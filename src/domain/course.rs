use super::*;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Course {
    pub name: String,
    pub sections: Vec<Section>,
    pub active_section: Option<usize>,
}

impl Course {
    pub fn course() -> Self {
        Course {
            name: "Jzero".into(),
            sections: vec![
                Section::vocabulary_group_a(),
                Section::vocabulary_group_b(),
            ],
            active_section: None,
        }
    }
}
