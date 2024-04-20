use reqwest_middleware::ClientWithMiddleware;
use serde::{ Deserialize, Serialize };
use std::collections::HashMap;

use crate::utils::get_response;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct MagicOrbResponse {
    pub data: MagicOrbResponseObject,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct MagicOrbResponseObject {
    #[serde(alias = "equipItem")]
    pub equip_item: EquipItem,
    #[serde(alias = "activeDeck")]
    pub active_deck: i16,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EquipItem {
    HashMap(HashMap<String, HashMap<String, MagicOrb>>),
    EmptyArray(Vec<()>),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct MagicOrb {
    #[serde(alias = "itemIdx")]
    pub item_idx: String,
    #[serde(alias = "itemLv")]
    pub item_level: i32,
    #[serde(alias = "itemExp")]
    pub item_exp: i32,
    pub grade: String,
    pub tier: String,
    #[serde(alias = "itemName")]
    pub item_name: String,
    #[serde(alias = "itemPath")]
    pub item_path: String,
}

pub async fn get_nft_magic_orb(
    transport_id: i32,
    client: ClientWithMiddleware
) -> anyhow::Result<MagicOrbResponse> {
    let request_url = format!(
        "https://webapi.mir4global.com/nft/character/magicorb?transportID={transport_id}&languageCode=en",
        transport_id = transport_id
    );

    let response_json: MagicOrbResponse = get_response(&client, request_url).await?;

    Ok(response_json)
}
