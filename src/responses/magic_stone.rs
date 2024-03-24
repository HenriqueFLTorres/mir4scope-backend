use serde::{ Deserialize, Serialize };
use std::collections::HashMap;
use crate::Nft;
use mongodb::{ bson, bson::doc, Collection, Database };

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
    pub active_deck: u8,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EquipItem {
    HashMap(HashMap<String, HashMap<String, MagicStone>>),
    EmptyArray(Vec<()>),
}

#[derive(Serialize, Deserialize, Debug)]
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
}

pub async fn get_nft_magic_stone(
    nft_collection: &Collection<Nft>,
    transport_id: &serde_json::Value,
    client: &reqwest::Client,
    database: &Database
) -> anyhow::Result<()> {
    let request_url = format!(
        "https://webapi.mir4global.com/nft/character/magicstone?transportID={transport_id}&languageCode=en",
        transport_id = transport_id
    );

    let response = client.get(request_url).send().await?.text().await?;
    let response_json: MagicStoneResponse = serde_json::from_str(&response)?;

    let magic_stone_collection = database.collection("Magic Stone");

    let record = magic_stone_collection.insert_one(response_json.data, None).await?;
    let filter = doc! { "transport_id": bson::to_bson(transport_id)? };
    let update = doc! { "$set": { "magic_stone_id": record.inserted_id.as_object_id() } };

    nft_collection.update_one(filter, update, None).await?;

    Ok(())
}
