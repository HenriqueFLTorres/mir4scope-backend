use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::responses::equip_item::EquipItem;
use crate::responses::nft::Character;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct Summary {
    pub character: Character,
    #[serde(alias = "tradeType")]
    pub trade_type: u8,
    #[serde(alias = "equipItem")]
    pub equip_items: HashMap<String, EquipItem>,
}