use serde::{ Deserialize, Serialize };
use std::collections::HashMap;
use crate::Nft;
use mongodb::{ bson, bson::doc, Collection };

#[derive(Serialize, Deserialize, Debug)]
pub struct BuildingResponse {
    pub code: u16,
    pub data: HashMap<String, BuildingObject>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BuildingObject {
    #[serde(alias = "buildingName")]
    pub building_name: String,
    #[serde(alias = "buildingLevel")]
    pub building_level: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct Building {
    pub building: HashMap<String, String>,
}

pub async fn get_nft_buildings(
    nft_collection: &Collection<Nft>,
    transport_id: &serde_json::Value,
    client: &reqwest::Client
) -> anyhow::Result<()> {
    let request_url = format!(
        "https://webapi.mir4global.com/nft/character/building?transportID={transport_id}&languageCode=en",
        transport_id = transport_id
    );

    let response = client.get(request_url).send().await?.text().await?;

    let response_json: BuildingResponse = serde_json::from_str(&response).unwrap();
    let building_hashmap: HashMap<String, String> = response_json.data
        .iter()
        .map(|building_object| {
            (building_object.1.building_name.clone(), building_object.1.building_level.clone())
        })
        .collect();

    let building_to_db: Building = Building {
        building: building_hashmap,
    };

    let filter = doc! { "transport_id": bson::to_bson(transport_id)? };
    let update = doc! { "$set": bson::to_bson(&building_to_db)? };

    nft_collection.update_one(filter, update, None).await?;

    Ok(())
}
