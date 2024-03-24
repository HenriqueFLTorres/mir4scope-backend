use crate::responses::equip_item::EquipItem;
use crate::responses::nft::Character;
use serde::{ Deserialize, Serialize };
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct Summary {
    pub character: Character,
    #[serde(alias = "tradeType")]
    pub trade_type: u8,
    #[serde(alias = "equipItem")]
    pub equip_items: HashMap<String, EquipItem>,
}
