use crate::utils::object_id;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct SpiritsResponse {
    pub data: SpiritsObject,
    #[serde(alias = "nftID")]
    #[serde(default = "object_id")]
    pub nft_id: mongodb::bson::oid::ObjectId,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct SpiritsObject {
    pub inven: Vec<Spirits>,
    pub equip: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct Spirits {
    pub transcend: i32,
    pub grade: i32,
    #[serde(alias = "petName")]
    pub pet_name: String,
    #[serde(alias = "iconPath")]
    pub icon_path: String,
}
