use super::*;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Course {
    sections: Vec<Section>,
    active_section: Option<Section>,
}



