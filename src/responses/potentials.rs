use crate::utils::object_id;
use crate::Nft;
use mongodb::{bson, bson::doc, Collection};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct PotentialsResponse {
    pub data: Potentials,
    #[serde(alias = "nftID")]
    #[serde(default = "object_id")]
    pub nft_id: mongodb::bson::oid::ObjectId,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct Potentials {
    total: i32,
    #[serde(alias = "totalMax")]
    total_max: i32,
    hunting: i32,
    #[serde(alias = "huntingMax")]
    hunting_max: i32,
    pvp: i32,
    #[serde(alias = "pvpMax")]
    pvp_max: i32,
    secondary: i32,
    #[serde(alias = "secondaryMax")]
    secondary_max: i32,
}

pub async fn get_nft_potentials(
    nft_collection: Collection<Nft>,
    transport_id: serde_json::Value,
    client: reqwest::Client,
) -> anyhow::Result<()> {
    let request_url = format!(
        "https://webapi.mir4global.com/nft/character/potential?transportID={transport_id}&languageCode=en",
        transport_id = transport_id
    );

    let response = client.get(request_url).send().await?.text().await?;
    let response_json: PotentialsResponse = serde_json::from_str(&response)?;

    let filter = doc! { "transport_id": bson::to_bson(&transport_id)? };
    let update = doc! { "$set": { "potentials": bson::to_bson(&response_json.data)?} };

    nft_collection.update_one(filter, update, None).await?;

    Ok(())
}
