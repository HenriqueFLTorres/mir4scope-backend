use std::collections::HashMap;
use reqwest_middleware::ClientWithMiddleware;
use serde::{Deserialize, Serialize};

use crate::utils::default_bool;
use crate::utils::default_hashmap;
use crate::utils::get_response;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InventoryResponse {
    #[serde(alias = "data")]
    pub inventory: Vec<InventoryItem>,
    #[serde(default = "default_hashmap")]
    pub craft_materials: HashMap<String, i32>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InventoryItem {
    #[serde(alias = "itemUID")]
    pub item_uid: String,
    #[serde(alias = "itemID")]
    pub item_id: String,
    pub enhance: u8,
    pub stack: i32,
    #[serde(alias = "tranceStep")]
    pub trance_step: u8,
    #[serde(alias = "RefineStep")]
    pub refine_step: u8,
    pub grade: String,
    #[serde(alias = "mainType")]
    pub main_type: u8,
    #[serde(alias = "subType")]
    pub sub_type: u8,
    #[serde(alias = "tabCategory")]
    pub tab_category: u8,
    pub tier: String,
    #[serde(alias = "itemName")]
    pub item_name: String,
    #[serde(alias = "itemPath")]
    pub item_path: String,
    #[serde(default = "default_bool")]
    pub is_tradable: bool,
}

pub async fn get_nft_inventory(
    transport_id: i32,
    client: ClientWithMiddleware,
    tradable_list: serde_json::Value,
) -> anyhow::Result<InventoryResponse> {
    let request_url = format!(
        "https://webapi.mir4global.com/nft/character/inven?transportID={transport_id}&languageCode=en",
        transport_id = transport_id
    );

    let mut response_json: InventoryResponse = get_response(&client, request_url).await?;
    
    response_json
        .inventory
        .iter_mut()
        .for_each(|i| {
            if tradable_list[&i.item_id] == 1 {
                i.is_tradable = true;
            }

// subtypes codes of items that are crafting materials
            let valid_sub_types = [3,4,5,6,7];

            if i.main_type == 9 && valid_sub_types.contains(&i.sub_type) {
                response_json.craft_materials.insert(i.item_name.clone(), i.stack);
            }
        });

    Ok(response_json)
}
