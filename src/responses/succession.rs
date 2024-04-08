use mongodb::{bson, Collection, Database};
use mongodb::bson::doc;
use serde::{ Deserialize, Serialize };
use std::collections::HashMap;

use super::item_detail::{ItemDetail, ItemDetailAdd};
use super::{inventory::InventoryItem, item_detail::get_item_detail, nft::Nft};

#[derive(Serialize, Deserialize, Debug)]
pub struct SuccessionResponse {
    pub data: SuccessionDataResponse,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SuccessionDataResponse {
    #[serde(alias = "equipItem")]
    pub equip_item: HashMap<String, SuccessionObject>,
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
    pub power_score: u32,
    #[serde(default)]
    pub options: Vec<ItemDetail>,
    #[serde(alias = "addOptions", default)]
    pub add_option: Vec<ItemDetailAdd>,
}

pub async fn get_nft_succession(
    nft_collection: Collection<Nft>,
    transport_id: u32,
    class: u32,
    client: reqwest::Client,
    database: Database,
    inventory: Vec<InventoryItem>
) -> anyhow::Result<()> {
    let request_url = format!(
        "https://webapi.mir4global.com/nft/character/succession?transportID={transport_id}&languageCode=en",
        transport_id = transport_id
    );

    let response = client.get(request_url).send().await?.text().await?;
    let response_json: SuccessionResponse = serde_json::from_str(&response)?;

    let mut succession_decks: HashMap<String,  SuccessionObject> = HashMap::new();

    for (slot_index, mut succession_value) in response_json.data.equip_item.clone().into_iter() {
        let item_match = inventory
            .iter()
            .find(|inventory_item| inventory_item.item_id == succession_value.item_idx)
            .expect("Succession not found in inventory");
        let item_details = get_item_detail(
            &client,
            &transport_id,
            &class,
            &item_match.item_uid
        ).await.expect("Succession item detail failed");

        succession_value.options = item_details.options;
        succession_value.add_option = item_details.add_option;
        succession_value.power_score = item_details.power_score;

        succession_decks.insert(slot_index, succession_value);
    }

    let succession_collection = database.collection("succession");
    let succession_to_db =
        doc! { "equip_item": bson::to_bson(&succession_decks)? };

    let record = succession_collection.insert_one(succession_to_db, None).await?;
    let filter = doc! { "transport_id": bson::to_bson(&transport_id)? };
    let update = doc! { "$set": { "succession_id": record.inserted_id.as_object_id() } };

    nft_collection.update_one(filter, update, None).await?;

    Ok(())
}
