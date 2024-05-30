use reqwest_middleware::ClientWithMiddleware;
use serde::{Deserialize, Serialize};

use crate::utils::get_response;
use crate::utils::default_bool;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InventoryResponse {
    #[serde(alias = "data")]
    pub inventory: Vec<InventoryItem>,
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
    pub is_tradable: bool
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
        .filter(|i| tradable_list[&i.item_id] == 1)
        .for_each(|i| {
            i.is_tradable = true;
        });

    Ok(response_json)
}
