use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct BuildingResponse {
    pub code: u16,
    pub data: HashMap<String, BuildingObject>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BuildingObject {
    #[serde(alias = "buildingName")]
    pub building_name: String,
    #[serde(alias = "buildingLevel")]
    pub building_level: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct Building {
    pub building: HashMap<String, String>,
}
