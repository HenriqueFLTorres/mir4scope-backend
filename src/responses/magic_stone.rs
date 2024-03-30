use super::{
    inventory::InventoryItem,
    item_detail::{ get_item_detail, ItemDetail, ItemDetailAdd },
};
use crate::Nft;
use mongodb::{ bson, bson::doc, Collection, Database };
use serde::{ Deserialize, Serialize };
use std::collections::HashMap;

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

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    #[serde(alias = "powerScore", default)]
    pub power_score: u32,
    #[serde(default)]
    pub options: Vec<ItemDetail>,
    #[serde(alias = "addOptions", default)]
    pub add_option: Vec<ItemDetailAdd>,
}

pub async fn get_nft_magic_stone(
    nft_collection: Collection<Nft>,
    transport_id: u32,
    class: u32,
    client: reqwest::Client,
    database: Database,
    inventory: Vec<InventoryItem>
) -> anyhow::Result<()> {
    let request_url = format!(
        "https://webapi.mir4global.com/nft/character/magicstone?transportID={transport_id}&languageCode=en",
        transport_id = transport_id
    );

    let response = client.get(request_url).send().await?.text().await?;
    let response_json: MagicStoneResponse = serde_json::from_str(&response)?;

    let mut magic_stones_decks: HashMap<String, HashMap<String, MagicStone>> = HashMap::new();
    for (set_index, inner_hashmap) in response_json.data.equip_item.clone().into_iter() {
        let mut magic_stones: HashMap<String, MagicStone> = HashMap::new();
        for (slot_index, mut stone_value) in inner_hashmap.clone().into_iter() {
            let item_match = inventory
                .iter()
                .find(|inventory_item| inventory_item.item_id == stone_value.item_idx)
                .expect("Magic stone not found in inventory.");
            let item_detail = get_item_detail(
                &client,
                &transport_id,
                &class,
                &item_match.item_uid
            ).await.expect("Magic stone item detail failed");

            stone_value.options = item_detail.options;
            stone_value.add_option = item_detail.add_option;
            stone_value.power_score = item_detail.power_score;

            magic_stones.insert(slot_index, stone_value);
        }
        magic_stones_decks.insert(set_index, magic_stones);
    }

    let magic_stone_collection = database.collection("Magic Stone");
    let magic_stone_to_db =
        doc! { "equip_item": bson::to_bson(&magic_stones_decks)?, "active_deck": bson::to_bson(&response_json.data.active_deck)? };

    let record = magic_stone_collection.insert_one(magic_stone_to_db, None).await?;
    let filter = doc! { "transport_id": bson::to_bson(&transport_id)? };
    let update = doc! { "$set": { "magic_stone_id": record.inserted_id.as_object_id() } };

    nft_collection.update_one(filter, update, None).await?;

    Ok(())
}
