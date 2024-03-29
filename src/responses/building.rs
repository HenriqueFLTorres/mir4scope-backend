use mongodb::bson::doc;
use serde::{ Deserialize, Serialize };
use std::collections::HashMap;

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

pub async fn get_nft_buildings(
    transport_id: serde_json::Value,
    client: reqwest::Client
) -> anyhow::Result<HashMap<String, String>> {
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

    Ok(building_hashmap)
}
