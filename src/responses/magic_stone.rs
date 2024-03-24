use serde::{ Deserialize, Serialize };
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct MagicStoneResponse {
    pub data: MagicStoneResponseObject,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct MagicStoneResponseObject {
    #[serde(alias = "equipItem")]
    pub equip_item: HashMap<String, HashMap<String, MagicStone>>,
    #[serde(alias = "activeDeck")]
    pub active_deck: u8,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EquipItem {
    HashMap(HashMap<String, HashMap<String, MagicStone>>),
    EmptyArray(Vec<()>),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct MagicStone {
    #[serde(alias = "itemIdx")]
    pub item_idx: String,
    #[serde(alias = "tranceStep")]
    pub trance_step: u8,
    #[serde(alias = "RefineStep")]
    pub refine_step: u8,
    pub grade: String,
    pub tier: String,
    #[serde(alias = "itemName")]
    pub item_name: String,
    #[serde(alias = "itemPath")]
    pub item_path: String,
}
