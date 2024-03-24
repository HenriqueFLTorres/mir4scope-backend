use serde::{ Deserialize, Serialize };
use std::collections::HashMap;
use crate::Nft;
use mongodb::{ bson, bson::doc, Collection, Database };

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct MysticalPieceResponse {
    pub data: MysticalPieceResponseObject,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct MysticalPieceResponseObject {
    #[serde(alias = "equipItem")]
    pub equip_item: HashMap<String, HashMap<String, MysticalPiece>>,
    #[serde(alias = "activeDeck")]
    pub active_deck: u8,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EquipItem {
    HashMap(HashMap<String, HashMap<String, MysticalPiece>>),
    EmptyArray(Vec<()>),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct MysticalPiece {
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

pub async fn get_nft_mystical_piece(
    nft_collection: &Collection<Nft>,
    transport_id: &serde_json::Value,
    client: &reqwest::Client,
    database: &Database
) -> anyhow::Result<()> {
    let request_url = format!(
        "https://webapi.mir4global.com/nft/character/mysticalpiece?transportID={transport_id}&languageCode=en",
        transport_id = transport_id
    );

    let response = client.get(request_url).send().await?.text().await?;
    let response_json: MysticalPieceResponse = serde_json::from_str(&response)?;

    let mystical_piece_collection = database.collection("Mystical Piece");

    let record = mystical_piece_collection.insert_one(response_json.data, None).await?;
    let filter = doc! { "transport_id": bson::to_bson(transport_id)? };
    let update = doc! { "$set": { "mystical_piece_id": record.inserted_id.as_object_id() } };

    nft_collection.update_one(filter, update, None).await?;

    Ok(())
}
