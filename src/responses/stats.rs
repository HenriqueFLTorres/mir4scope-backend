use crate::Nft;
use mongodb::{ bson, bson::doc, Collection };
use serde::{ Deserialize, Serialize };
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct StatsResponse {
    pub data: StatsObject,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct StatsObject {
    pub lists: Vec<Stats>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct Stats {
    #[serde(alias = "statName")]
    pub stat_name: String,
    #[serde(alias = "statValue")]
    pub stat_value: String,
    #[serde(alias = "iconPath")]
    pub icon_path: String,
}

pub async fn get_nft_stats(
    nft_collection: Collection<Nft>,
    transport_id: serde_json::Value,
    client: reqwest::Client,
) -> anyhow::Result<()> {
    let request_url = format!(
        "https://webapi.mir4global.com/nft/character/stats?transportID={transport_id}&languageCode=en",
        transport_id = transport_id
    );

    let response = client.get(request_url).send().await?.text().await?;

    let response_json: StatsResponse = serde_json::from_str(&response)?;
    let stats_hashmap: HashMap<String, String> = response_json.data.lists
        .iter()
        .map(|stats_object| { (stats_object.stat_name.clone(), stats_object.stat_value.clone()) })
        .collect();

    let filter = doc! { "transport_id": bson::to_bson(&transport_id)? };
    let update = doc! { "$set": { "stats": bson::to_bson(&stats_hashmap)? } };

    nft_collection.update_one(filter, update, None).await?;

    Ok(())
}
