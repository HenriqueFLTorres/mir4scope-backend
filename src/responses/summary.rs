use crate::responses::item_detail::{ get_item_detail, ItemDetail };
use crate::responses::nft::Nft;
use mongodb::{ bson, bson::doc, Collection };
use serde::{ Deserialize, Serialize };
use std::collections::HashMap;

use super::{ inventory::InventoryItem, item_detail::ItemDetailAdd };

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct SummaryResponse {
    pub data: SummaryResponseObject,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct SummaryResponseObject {
    pub character: Character,
    #[serde(alias = "tradeType")]
    pub trade_type: u8,
    #[serde(alias = "equipItem")]
    pub equip_items: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct Character {
    #[serde(alias = "worldName")]
    pub world_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct EquipItem {
    #[serde(alias = "itemIdx")]
    pub item_idx: String,
    pub enhance: String,
    #[serde(alias = "refineStep")]
    pub refine_step: String,
    pub grade: String,
    pub tier: String,
    #[serde(alias = "itemType")]
    pub item_type: String,
    #[serde(alias = "itemName")]
    pub item_name: String,
    #[serde(alias = "itemPath")]
    pub item_path: String,
    #[serde(alias = "powerScore")]
    pub power_score: u32,
    pub options: Vec<ItemDetail>,
    #[serde(alias = "addOptions")]
    pub add_option: Vec<ItemDetailAdd>,
}

pub async fn get_nft_summary(
    nft_collection: Collection<Nft>,
    seq: u32,
    transport_id: u32,
    class: u32,
    client: reqwest::Client,
    inventory: Vec<InventoryItem>
) -> anyhow::Result<()> {
    let request_url = format!(
        "https://webapi.mir4global.com/nft/character/summary?seq={seq}&languageCode=en",
        seq = seq
    );

    let response = client.get(request_url).send().await?.text().await?;
    let mut response_json: SummaryResponse = serde_json::from_str(&response)?;

    let mut equip_items: HashMap<String, EquipItem> = HashMap::new();
    for (key, value) in response_json.data.equip_items.clone().into_iter() {
        let item_match = inventory
            .iter()
            .find(|inventory_item| inventory_item.item_id == value["itemIdx"])
            .expect("Item not found in inventory.");
        let item_detail = get_item_detail(
            &client,
            &transport_id,
            &class,
            &item_match.item_uid
        ).await.expect("item detail failed");

        response_json.data.equip_items.entry(key.clone()).and_modify(|equip_item| {
            equip_item["options"] = serde_json::to_value(item_detail.options).unwrap();
            equip_item["add_option"] = serde_json::to_value(item_detail.add_option).unwrap();
            equip_item["power_score"] = serde_json::to_value(item_detail.power_score).unwrap();
        });
        let equip_object: EquipItem = serde_json::from_value(
            response_json.data.equip_items[&key].clone()
        )?;

        equip_items.insert(key, equip_object);
    }

    let filter = doc! { "seq": bson::to_bson(&seq)? };
    let update =
        doc! { "$set": { "trade_type": bson::to_bson(&response_json.data.trade_type)?, "world_name": bson::to_bson(&response_json.data.character.world_name)?, "equip_items": bson::to_bson(&equip_items)? } };

    nft_collection.update_one(filter, update, None).await?;

    Ok(())
}
