use std::collections::HashMap;

use crate::utils::object_id;
use crate::Nft;
use mongodb::{ bson, bson::doc, Collection, Database };
use serde::{ Deserialize, Serialize };

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
    pub inven: Vec<Spirit>,
    pub equip: HashMap<String, HashMap<String, Spirit>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct Spirit {
    pub transcend: i32,
    pub grade: i32,
    #[serde(alias = "petName")]
    pub pet_name: String,
    #[serde(alias = "iconPath")]
    pub icon_path: String,
}

pub async fn get_nft_spirits(
    nft_collection: Collection<Nft>,
    transport_id: u32,
    client: reqwest::Client,
    database: Database
) -> anyhow::Result<()> {
    let request_url = format!(
        "https://webapi.mir4global.com/nft/character/spirit?transportID={transport_id}&languageCode=en",
        transport_id = transport_id
    );

    let response = client.get(request_url).send().await?.text().await?;
    let response_json: SpiritsResponse = serde_json::from_str(&response)?;

    let spirits_collection = database.collection("Spirits");
    let data_to_db =
        doc! { "equip": bson::to_bson(&response_json.data.equip)?, "inven": bson::to_bson(&response_json.data.inven)? };

    let record = spirits_collection.insert_one(data_to_db, None).await?;
    let filter = doc! { "transport_id": bson::to_bson(&transport_id)? };
    let update = doc! { "$set": { "spirits_id": record.inserted_id.as_object_id() } };

    nft_collection.update_one(filter, update, None).await?;

    Ok(())
}
