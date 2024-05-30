use crate::utils::get_response;

use super::{
    inventory::InventoryItem,
    item_detail::{get_item_detail, ItemDetail, ItemDetailAdd},
};

use crate::utils::default_bool;

use reqwest_middleware::ClientWithMiddleware;
use serde::{Deserialize, Serialize};
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
    pub active_deck: i16,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EquipItem {
    HashMap(HashMap<String, HashMap<String, MagicStone>>),
    EmptyArray(Vec<()>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    #[serde(alias = "powerScore", default)]
    pub power_score: i32,
    #[serde(default)]
    pub options: Vec<ItemDetail>,
    #[serde(alias = "addOptions", default)]
    pub add_option: Vec<ItemDetailAdd>,
    #[serde(default = "default_bool")]
    pub is_tradable: bool
}

pub async fn get_nft_magic_stone(
    transport_id: i32,
    class: i32,
    client: ClientWithMiddleware,
    inventory: Vec<InventoryItem>,
    tradable_list: serde_json::Value,
) -> anyhow::Result<MagicStoneResponseObject> {
    let request_url = format!(
        "https://webapi.mir4global.com/nft/character/magicstone?transportID={transport_id}&languageCode=en",
        transport_id = transport_id
    );

    let response_json: MagicStoneResponse = get_response(&client, request_url).await?;

    let mut magic_stones_decks: HashMap<String, HashMap<String, MagicStone>> = HashMap::new();
    for (set_index, inner_hashmap) in response_json.data.equip_item.clone().into_iter() {
        let mut magic_stones: HashMap<String, MagicStone> = HashMap::new();
        for (slot_index, mut stone_value) in inner_hashmap.clone().into_iter() {
            let item_match = inventory
                .iter()
                .find(|inventory_item| inventory_item.item_id == stone_value.item_idx);

            if item_match.is_some() {
                let item_detail = get_item_detail(
                    &client,
                    &transport_id,
                    &class,
                    &item_match.unwrap().item_uid,
                )
                .await
                .expect("Magic stone item detail failed");

                stone_value.options = item_detail.options;
                stone_value.add_option = item_detail.add_option;
                stone_value.power_score = item_detail.power_score;
                
                if tradable_list[&stone_value.item_idx] == 1 {
                    stone_value.is_tradable = true
                }
            } else {
                println!("Inventory magic stone item match not found");
                stone_value.options = Vec::new();
                stone_value.add_option = Vec::new();
                stone_value.power_score = 0;
            }

            magic_stones.insert(slot_index, stone_value);
        }
        magic_stones_decks.insert(set_index, magic_stones);
    }

    let magic_stone_result = MagicStoneResponseObject {
        equip_item: magic_stones_decks,
        active_deck: response_json.data.active_deck,
    };

    Ok(magic_stone_result)
}
