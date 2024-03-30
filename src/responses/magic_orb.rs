use crate::Nft;
use mongodb::{ bson, bson::doc, Collection, Database };
use serde::{ Deserialize, Serialize };
use std::collections::HashMap;

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
    pub active_deck: u8,
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
    nft_collection: Collection<Nft>,
    transport_id: u32,
    client: reqwest::Client,
    database: Database
) -> anyhow::Result<()> {
    let request_url = format!(
        "https://webapi.mir4global.com/nft/character/magicorb?transportID={transport_id}&languageCode=en",
        transport_id = transport_id
    );

    let response = client.get(request_url).send().await?.text().await?;
    let response_json: MagicOrbResponse = serde_json::from_str(&response)?;

    let magic_orb_collection = database.collection("Magic Orb");

    let record = magic_orb_collection.insert_one(response_json.data, None).await?;
    let filter = doc! { "transport_id": bson::to_bson(&transport_id)? };
    let update = doc! { "$set": { "magic_orb_id": record.inserted_id.as_object_id() } };

    nft_collection.update_one(filter, update, None).await?;

    Ok(())
}
