use reqwest_middleware::ClientWithMiddleware;
use serde::{ Deserialize, Serialize };
use std::collections::HashMap;

use crate::utils::get_response;

#[derive(Serialize, Deserialize, Debug)]
pub struct SuccessionResponse {
    pub data: SuccessionDataResponse,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SuccessionDataResponse {
    #[serde(alias = "equipItem")]
    pub equip_item: EquipItem,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize, Debug)]
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
}

pub async fn get_nft_succession(
    transport_id: i32,
    client: ClientWithMiddleware
) -> anyhow::Result<SuccessionResponse> {
    let request_url = format!(
        "https://webapi.mir4global.com/nft/character/succession?transportID={transport_id}&languageCode=en",
        transport_id = transport_id
    );

    let response_json: SuccessionResponse = get_response(&client, request_url).await?;

    Ok(response_json)
}
