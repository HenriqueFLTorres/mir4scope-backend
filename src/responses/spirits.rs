use std::collections::HashMap;

use reqwest_middleware::ClientWithMiddleware;
use serde::{Deserialize, Serialize};

use crate::utils::get_response;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct SpiritsResponse {
    pub data: SpiritsObject,
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
    transport_id: i32,
    client: ClientWithMiddleware,
) -> anyhow::Result<SpiritsObject> {
    let request_url = format!(
        "https://webapi.mir4global.com/nft/character/spirit?transportID={transport_id}&languageCode=en",
        transport_id = transport_id
    );

    let response_json: SpiritsResponse = get_response(&client, request_url).await?;

    Ok(response_json.data)
}
