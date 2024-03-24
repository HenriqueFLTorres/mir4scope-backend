use serde::{ Deserialize, Serialize };
use std::collections::HashMap;
use crate::Nft;
use mongodb::{ bson, bson::doc, Collection };

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
    pub equip_items: HashMap<String, EquipItem>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct Character {
    #[serde(alias = "worldName")]
    pub world_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
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
}

pub async fn get_nft_summary(
    nft_collection: &Collection<Nft>,
    seq: &serde_json::Value,
    client: &reqwest::Client
) -> anyhow::Result<()> {
    let request_url = format!(
        "https://webapi.mir4global.com/nft/character/summary?seq={seq}&languageCode=en",
        seq = seq
    );

    let response = client.get(request_url).send().await?.text().await?;
    let response_json: SummaryResponse = serde_json::from_str(&response)?;

    let filter = doc! { "seq": bson::to_bson(seq)? };
    let update =
        doc! { "$set": { "trade_type": bson::to_bson(&response_json.data.trade_type)?, "world_name": bson::to_bson(&response_json.data.character.world_name)?, "equip_items": bson::to_bson(&response_json.data.equip_items)? } };

    nft_collection.update_one(filter, update, None).await?;

    Ok(())
}
