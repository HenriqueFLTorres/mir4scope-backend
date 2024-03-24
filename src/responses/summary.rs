use crate::responses::equip_item::EquipItem;
use crate::responses::nft::Character;
use serde::{ Deserialize, Serialize };
use std::collections::HashMap;
use crate::Nft;
use mongodb::{ bson, bson::doc, Collection };

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct Summary {
    pub character: Character,
    #[serde(alias = "tradeType")]
    pub trade_type: u8,
    #[serde(alias = "equipItem")]
    pub equip_items: HashMap<String, EquipItem>,
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

    let response = client.get(request_url).send().await?;
    let json: serde_json::Value = response.json().await?;
    let data = &json["data"];

    let filter = doc! { "seq": bson::to_bson(seq)? };
    let update =
        doc! { "$set": { "trade_type": bson::to_bson(&data["tradeType"])?, "world_name": bson::to_bson(&data["character"]["worldName"])?, "equip_items": bson::to_bson(&data["equipItem"])? } };

    nft_collection.update_one(filter, update, None).await?;

    Ok(())
}
