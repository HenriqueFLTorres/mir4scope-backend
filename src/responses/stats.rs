use serde::{Deserialize, Serialize};
use crate::utils::object_id;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct StatsResponse {
    pub data: StatsData,
    #[serde(alias = "nftID")]
    #[serde(default = "object_id")]
    pub nft_id: mongodb::bson::oid::ObjectId,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct StatsData {
    pub lists: Vec<Stats>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct Stats {
    #[serde(alias = "statName")]
    pub stat_name: String,
    #[serde(alias = "statValue")]
    pub stat_value: String,
    #[serde(alias = "iconPath")]
    pub icon_path: String,
}