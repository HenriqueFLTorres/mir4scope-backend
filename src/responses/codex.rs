use std::collections::HashMap;

use mongodb::bson::doc;
use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct CodexResponse {
    pub data: HashMap<String, Codex>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct Codex {
    #[serde(alias = "codexName")]
    pub codex_name: String,
    #[serde(alias = "totalCount")]
    pub total_count: String,
    pub completed: String,
    #[serde(alias = "inprogress")]
    pub in_progress: String,
}

pub async fn get_nft_codex(
    transport_id: u32,
    client: reqwest::Client
) -> anyhow::Result<HashMap<String, Codex>> {
    let request_url = format!(
        "https://webapi.mir4global.com/nft/character/codex?transportID={transport_id}&languageCode=en",
        transport_id = transport_id
    );

    let response = client.get(request_url).send().await?.text().await?;
    let response_json: CodexResponse = serde_json::from_str(&response)?;

    Ok(response_json.data)
}
