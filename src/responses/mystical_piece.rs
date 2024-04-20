use reqwest_middleware::ClientWithMiddleware;
use serde::{ Deserialize, Serialize };
use std::collections::HashMap;

use crate::utils::get_response;

use super::{
    inventory::InventoryItem,
    item_detail::{ self, get_item_detail, ItemDetail, ItemDetailAdd, ItemDetailData },
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
    pub active_deck: i16,
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
    pub power_score: i32,
    #[serde(default)]
    pub options: Vec<ItemDetail>,
    #[serde(alias = "addOptions", default)]
    pub add_option: Vec<ItemDetailAdd>,
}

pub async fn get_nft_mystical_piece(
    transport_id: i32,
    class: i32,
    client: ClientWithMiddleware,
    inventory: Vec<InventoryItem>
) -> anyhow::Result<MysticalPieceResponseObject> {
    let request_url = format!(
        "https://webapi.mir4global.com/nft/character/mysticalpiece?transportID={transport_id}&languageCode=en",
        transport_id = transport_id
    );

    let response_json: MysticalPieceResponse = get_response(&client, request_url).await?;

    let mut mystical_pieces_decks: HashMap<String, HashMap<String, MysticalPiece>> = HashMap::new();
    for (set_index, inner_hashmap) in response_json.data.equip_item.clone().into_iter() {
        let mut mystical_pieces: HashMap<String, MysticalPiece> = HashMap::new();
        for (slot_index, mut piece_value) in inner_hashmap.clone().into_iter() {
            let item_match = inventory
                .iter()
                .find(|inventory_item| inventory_item.item_id == piece_value.item_idx);

            if item_match.is_some() {
                let item_detail = get_item_detail(
                    &client,
                    &transport_id,
                    &class,
                    &item_match.unwrap().item_uid
                ).await.expect("Mystical piece item detail failed");

                piece_value.options = item_detail.options;
                piece_value.add_option = item_detail.add_option;
                piece_value.power_score = item_detail.power_score;
            } else {
                println!("Inventory mystical piece item match not found");
                piece_value.options = Vec::new();
                piece_value.add_option = Vec::new();
                piece_value.power_score = 0;
            }

            mystical_pieces.insert(slot_index, piece_value);
        }
        mystical_pieces_decks.insert(set_index, mystical_pieces);
    }

    let mystical_piece_result = MysticalPieceResponseObject {
        equip_item: mystical_pieces_decks,
        active_deck: response_json.data.active_deck,
    };

    Ok(mystical_piece_result)
}
