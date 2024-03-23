use crate::utils::object_id;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct AssetsResponse {
    pub data: Assets,
    #[serde(alias = "nftID")]
    #[serde(default = "object_id")]
    pub nft_id: mongodb::bson::oid::ObjectId,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct Assets {
    pub copper: String,
    pub energy: String,
    pub darksteel: String,
    pub speedups: String,
    pub dragonjade: String,
    pub acientcoins: String,
    pub dragonsteel: i32,
}
