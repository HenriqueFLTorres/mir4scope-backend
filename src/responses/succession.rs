use crate::Nft;
use mongodb::{bson, bson::doc, Collection};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    nft_collection: Collection<Nft>,
    transport_id: serde_json::Value,
    client: reqwest::Client,
) -> anyhow::Result<()> {
    let request_url = format!(
        "https://webapi.mir4global.com/nft/character/succession?transportID={transport_id}&languageCode=en",
        transport_id = transport_id
    );

    let response = client.get(request_url).send().await?.text().await?;
    let response_json: SuccessionResponse = serde_json::from_str(&response)?;

    let filter = doc! { "transport_id": bson::to_bson(&transport_id)? };
    let update = doc! { "$set": { "succession": bson::to_bson(&response_json.data.equip_item)? } };

    nft_collection.update_one(filter, update, None).await?;

    Ok(())
}
