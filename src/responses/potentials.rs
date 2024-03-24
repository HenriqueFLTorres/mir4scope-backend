use crate::utils::object_id;
use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct PotentialsResponse {
    pub data: Potentials,
    #[serde(alias = "nftID")]
    #[serde(default = "object_id")]
    pub nft_id: mongodb::bson::oid::ObjectId,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct Potentials {
    total: i32,
    #[serde(alias = "totalMax")]
    total_max: i32,
    hunting: i32,
    #[serde(alias = "huntingMax")]
    hunting_max: i32,
    pvp: i32,
    #[serde(alias = "pvpMax")]
    pvp_max: i32,
    secondary: i32,
    #[serde(alias = "secondaryMax")]
    secondary_max: i32,
}
