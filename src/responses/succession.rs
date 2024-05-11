use reqwest_middleware::ClientWithMiddleware;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::utils::get_response;

use super::{
    inventory::InventoryItem,
    item_detail::{get_item_detail, ItemDetail, ItemDetailAdd},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SuccessionResponse {
    pub data: SuccessionDataResponse,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SuccessionDataResponse {
    #[serde(alias = "equipItem")]
    pub equip_item: EquipItem,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum EquipItem {
    HashMap(HashMap<String, SuccessionObject>),
    EmptyArray(Vec<()>),
}

impl Default for EquipItem {
    fn default() -> Self {
        EquipItem::EmptyArray(Vec::new())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SuccessionObject {
    #[serde(alias = "itemIdx")]
    pub item_idx: String,
    #[serde(alias = "tranceStep")]
    pub trance_step: u8,
    #[serde(alias = "RefineStep")]
    pub refine_step: u8,
    pub enhance: u8,
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
}

pub async fn get_nft_succession(
    transport_id: i32,
    client: ClientWithMiddleware,
    class: i32,
    inventory: Vec<InventoryItem>,
) -> anyhow::Result<SuccessionResponse> {
    let request_url = format!(
        "https://webapi.mir4global.com/nft/character/succession?transportID={transport_id}&languageCode=en",
        transport_id = transport_id
    );

    let mut response_json: SuccessionResponse = get_response(&client, request_url).await?;

    match response_json.clone().data.equip_item {
        EquipItem::HashMap(item) => {
            let mut succession_items: HashMap<String, SuccessionObject> = HashMap::new();
            for (item_index, mut succession) in item.clone().into_iter() {
                let item_match = inventory
                    .iter()
                    .find(|inventory_item| inventory_item.item_id == succession.item_idx);

                if let Some(item) = item_match {
                    let item_detail =
                        get_item_detail(&client, &transport_id, &class, &item.item_uid)
                            .await
                            .expect("Succession item detail failed");
                    succession.options = item_detail.options;
                    succession.add_option = item_detail.add_option;
                    succession.power_score = item_detail.power_score;
                } else {
                    println!("Inventory succession item match not found");
                    succession.options = Vec::new();
                    succession.add_option = Vec::new();
                    succession.power_score = 0;
                }

                succession_items.insert(item_index, succession);
            }
            response_json.data.equip_item = EquipItem::HashMap(succession_items);
        }
        EquipItem::EmptyArray(_) => {}
    }

    Ok(response_json)
}
