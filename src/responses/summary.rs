use crate::{
    responses::item_detail::{get_item_detail, ItemDetail},
    utils::default_bool,
    utils::get_response,
};
use reqwest_middleware::ClientWithMiddleware;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{inventory::InventoryItem, item_detail::ItemDetailAdd};

#[derive(Serialize, Deserialize, Debug)]
pub struct SummaryResponse {
    pub data: SummaryResponseObject,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SummaryResponseObject {
    pub character: Character,
    #[serde(alias = "tradeType")]
    pub trade_type: i32,
    #[serde(alias = "equipItem")]
    pub equip_items: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SummaryReturnObject {
    pub world_name: String,
    pub trade_type: i32,
    pub equip_items: HashMap<String, EquipItem>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Character {
    #[serde(alias = "worldName")]
    pub world_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    #[serde(alias = "powerScore")]
    pub power_score: i32,
    pub options: Vec<ItemDetail>,
    #[serde(alias = "addOptions")]
    pub add_option: Vec<ItemDetailAdd>,
    #[serde(default = "default_bool")]
    pub is_tradable: bool,
}

pub async fn get_nft_summary(
    seq: i32,
    transport_id: i32,
    class: i32,
    client: ClientWithMiddleware,
    inventory: Vec<InventoryItem>,
    tradable_list: serde_json::Value,
) -> anyhow::Result<SummaryReturnObject> {
    let request_url = format!(
        "https://webapi.mir4global.com/nft/character/summary?seq={seq}&languageCode=en",
        seq = seq
    );

    let mut response_json: SummaryResponse = get_response(&client, request_url).await?;

    let mut equip_items: HashMap<String, EquipItem> = HashMap::new();
    for (key, value) in response_json.data.equip_items.clone().into_iter() {
        let item_match = inventory
            .iter()
            .find(|inventory_item| inventory_item.item_id == value["itemIdx"])
            .expect("Item not found in inventory.");
        let item_detail = get_item_detail(&client, &transport_id, &class, &item_match.item_uid)
            .await
            .expect("item detail failed");

        response_json
            .data
            .equip_items
            .entry(key.clone())
            .and_modify(|equip_item| {
                equip_item["options"] = serde_json::to_value(item_detail.options).unwrap();
                equip_item["add_option"] = serde_json::to_value(item_detail.add_option).unwrap();
                equip_item["power_score"] = serde_json::to_value(item_detail.power_score).unwrap();

                let item_id: String =
                    serde_json::from_value(equip_item["itemIdx"].clone()).unwrap();
                if tradable_list[&item_id.to_string()] == 1 {
                    equip_item["is_tradable"] = serde_json::to_value(true).unwrap();
                    println!("encoutronou");
                    println!("{:#?}", equip_item["is_tradable"])
                }
            });
        let equip_object: EquipItem =
            serde_json::from_value(response_json.data.equip_items[&key].clone())?;

        equip_items.insert(key, equip_object);
    }

    let summary_to_db: SummaryReturnObject = SummaryReturnObject {
        trade_type: response_json.data.trade_type,
        world_name: response_json.data.character.world_name,
        equip_items,
    };

    Ok(summary_to_db)
}
