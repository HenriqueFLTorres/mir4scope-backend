use crate::Nft;
use mongodb::{ bson, bson::doc, Collection, Database };
use serde::{ Deserialize, Serialize };
use std::collections::HashMap;

use super::{
    inventory::InventoryItem,
    item_detail::{ get_item_detail, ItemDetail, ItemDetailAdd },
};

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

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    #[serde(alias = "powerScore", default)]
    pub power_score: u32,
    #[serde(default)]
    pub options: Vec<ItemDetail>,
    #[serde(alias = "addOptions", default)]
    pub add_option: Vec<ItemDetailAdd>,
}

pub async fn get_nft_mystical_piece(
    nft_collection: Collection<Nft>,
    transport_id: u32,
    class: u32,
    client: reqwest::Client,
    database: Database,
    inventory: Vec<InventoryItem>
) -> anyhow::Result<()> {
    let request_url = format!(
        "https://webapi.mir4global.com/nft/character/mysticalpiece?transportID={transport_id}&languageCode=en",
        transport_id = transport_id
    );

    let response = client.get(request_url).send().await?.text().await?;
    let response_json: MysticalPieceResponse = serde_json::from_str(&response)?;

    let mut mystical_pieces_decks: HashMap<String, HashMap<String, MysticalPiece>> = HashMap::new();
    for (set_index, inner_hashmap) in response_json.data.equip_item.clone().into_iter() {
        let mut mystical_pieces: HashMap<String, MysticalPiece> = HashMap::new();
        for (slot_index, mut piece_value) in inner_hashmap.clone().into_iter() {
            let item_match = inventory
                .iter()
                .filter(|inventory_item| inventory_item.item_id == piece_value.item_idx)
                .next()
                .expect(format!("Mystical piece: {:#?} failed,\ntransport_id: {:#?}", piece_value, transport_id).as_str());

            let item_detail = get_item_detail(
                &client,
                &transport_id,
                &class,
                &item_match.item_uid
            ).await.expect("Mystical Piece item detail failed");

            piece_value.options = item_detail.options.clone();
            piece_value.add_option = item_detail.add_option.clone();
            piece_value.power_score = item_detail.power_score.clone();

            mystical_pieces.insert(slot_index, piece_value);
        }
        mystical_pieces_decks.insert(set_index, mystical_pieces.clone());
    }

    let mystical_piece_collection = database.collection("mystical_piece");
    let mystical_piece_to_db =
        doc! { "equip_item": bson::to_bson(&mystical_pieces_decks)?, "active_deck": bson::to_bson(&response_json.data.active_deck)? };

    let record = mystical_piece_collection.insert_one(mystical_piece_to_db, None).await?;
    let filter = doc! { "transport_id": bson::to_bson(&transport_id)? };
    let update = doc! { "$set": { "mystical_piece_id": record.inserted_id.as_object_id() } };

    nft_collection.update_one(filter, update, None).await?;

    Ok(())
}
