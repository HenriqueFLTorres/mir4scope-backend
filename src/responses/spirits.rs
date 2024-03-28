use crate::utils::object_id;
use crate::Nft;
use mongodb::{bson, bson::doc, Collection, Database};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct SpiritsResponse {
    pub data: SpiritsObject,
    #[serde(alias = "nftID")]
    #[serde(default = "object_id")]
    pub nft_id: mongodb::bson::oid::ObjectId,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct SpiritsObject {
    pub inven: Vec<Spirits>,
    pub equip: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct Spirits {
    pub transcend: i32,
    pub grade: i32,
    #[serde(alias = "petName")]
    pub pet_name: String,
    #[serde(alias = "iconPath")]
    pub icon_path: String,
}

pub async fn get_nft_spirits(
    nft_collection: Collection<Nft>,
    transport_id: serde_json::Value,
    client: reqwest::Client,
    database: Database,
) -> anyhow::Result<()> {
    let request_url = format!(
        "https://webapi.mir4global.com/nft/character/spirit?transportID={transport_id}&languageCode=en",
        transport_id = transport_id
    );

    let response = client.get(request_url).send().await?.text().await?;
    let response_json: SpiritsResponse = serde_json::from_str(&response)?;

    let spirits_collection = database.collection("Spirits");

    let record = spirits_collection.insert_one(response_json, None).await?;
    let filter = doc! { "transport_id": bson::to_bson(&transport_id)? };
    let update = doc! { "$set": { "spirits_id": record.inserted_id.as_object_id() } };

    nft_collection.update_one(filter, update, None).await?;

    Ok(())
}
