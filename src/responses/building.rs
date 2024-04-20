use reqwest_middleware::ClientWithMiddleware;
use serde::{ Deserialize, Serialize };
use std::collections::HashMap;

use crate::utils::get_response;

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
    transport_id: i32,
    client: ClientWithMiddleware
) -> anyhow::Result<HashMap<String, String>> {
    let request_url = format!(
        "https://webapi.mir4global.com/nft/character/building?transportID={transport_id}&languageCode=en",
        transport_id = transport_id
    );

    let response_json: BuildingResponse = get_response(&client, request_url).await?;
    
    let building_hashmap: HashMap<String, String> = response_json.data
        .iter()
        .map(|building_object| {
            (building_object.1.building_name.clone(), building_object.1.building_level.clone())
        })
        .collect();

    Ok(building_hashmap)
}
