use crate::utils::object_id;
use crate::Nft;
use mongodb::{bson, bson::doc, Collection, Database};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct StatsResponse {
    pub data: StatsObject,
    #[serde(alias = "nftID")]
    #[serde(default = "object_id")]
    pub nft_id: mongodb::bson::oid::ObjectId,
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
    database: Database,
) -> anyhow::Result<()> {
    let request_url = format!(
        "https://webapi.mir4global.com/nft/character/stats?transportID={transport_id}&languageCode=en",
        transport_id = transport_id
    );

    let response = client.get(request_url).send().await?.text().await?;

    let stats_json: StatsResponse = serde_json::from_str(&response)?;

    let stats_collection = database.collection("Stats");

    let record = stats_collection.insert_one(stats_json, None).await?;
    let filter = doc! { "transport_id": bson::to_bson(&transport_id)? };
    let update = doc! { "$set": { "stats_id": record.inserted_id.as_object_id()  } };

    nft_collection.update_one(filter, update, None).await?;

    Ok(())
}
