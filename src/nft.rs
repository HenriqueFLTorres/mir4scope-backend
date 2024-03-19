use mongodb::{ bson::{ doc } };
use serde::{ Deserialize, Serialize };
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct Nft {
    pub seq: u32,
    #[serde(alias = "transportID")]
    pub transport_id: u32,
    #[serde(alias = "nftID")]
    pub nft_id: String,
    #[serde(alias = "sealedDT")]
    pub sealed_dt: u32,
    #[serde(alias = "characterName")]
    pub character_name: String,
    pub class: u32,
    #[serde(alias = "lv")]
    pub lvl: u32,
    #[serde(alias = "powerScore")]
    pub power_score: u32,
    pub price: u32,
    #[serde(alias = "MirageScore")]
    pub mirage_score: u32,
    #[serde(alias = "MiraX")]
    pub mira_x: u32,
    #[serde(alias = "Reinforce")]
    pub reinforce: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct Summary {
    pub character: Character,
    #[serde(alias = "tradeType")]
    pub trade_type: u8,
    #[serde(alias = "equipItem")]
    pub equip_items: HashMap<String, EquipItem>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct Character {
    #[serde(alias = "worldName")]
    pub world_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct EquipItem {
    #[serde(alias = "itemIdx")]
    pub item_idx: String,
    pub enhance: String,
    #[serde(alias = "refineStep")]
    pub refine_step: String,
    pub grade: String,
    pub tier: String,
    #[serde(alias = "itemType")]
    pub item_type: String,
    #[serde(alias = "itemName")]
    pub item_name: String,
    #[serde(alias = "itemPath")]
    pub item_path: String,
}
