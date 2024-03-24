use serde::{ Deserialize, Serialize };
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct HolyStuffResponse {
    pub data: HashMap<String, HolyStuffObject>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HolyStuffObject {
    #[serde(alias = "HolyStuffName")]
    pub holy_stuff_name: String,
    #[serde(alias = "Grade")]
    pub grade: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct HolyStuff {
    pub holy_stuff: HashMap<String, String>,
}
