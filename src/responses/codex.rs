use std::collections::HashMap;

use mongodb::bson::doc;
use serde::{ Deserialize, Serialize };

use crate::responses;

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct CodexResponse {
    pub data: HashMap<String, Codex>,
    #[serde(default)]
    pub in_progress: u32,
    #[serde(default)]
    pub completed: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct Codex {
    #[serde(alias = "codexName")]
    pub codex_name: String,
    #[serde(alias = "totalCount")]
    pub total_count: StringOrU32,
    pub completed: StringOrU32,
    #[serde(alias = "inprogress")]
    pub in_progress: StringOrU32,
    #[serde(default)]
    pub pinto: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum StringOrU32 {
    String(String),
    Integer(u32),
}

impl StringOrU32 {
    pub fn as_u32(&self) -> Option<u32> {
        match self {
            StringOrU32::Integer(num) => Some(*num),
            StringOrU32::String(s) => s.parse().ok(),
        }
    }
}

pub async fn get_nft_codex(
    transport_id: u32,
    client: reqwest::Client
) -> anyhow::Result<CodexResponse> {
    let request_url = format!(
        "https://webapi.mir4global.com/nft/character/codex?transportID={transport_id}&languageCode=en",
        transport_id = transport_id
    );

    let response = client.get(request_url).send().await?.text().await?;
    let mut response_json: CodexResponse = serde_json::from_str(&response)?;

    let mut in_progress_total = 0;
    let mut completed_total = 0;
    response_json.data.iter_mut().for_each(|(_, codex)| {
        let in_progress_int: u32 = codex.in_progress.as_u32().unwrap();
        let completed_int: u32 = codex.completed.as_u32().unwrap();
        let total_count_int: u32 = codex.total_count.as_u32().unwrap();

        in_progress_total += in_progress_int;
        completed_total += completed_int;
        codex.in_progress = responses::codex::StringOrU32::Integer(in_progress_int);
        codex.completed = responses::codex::StringOrU32::Integer(completed_int);
        codex.total_count = responses::codex::StringOrU32::Integer(total_count_int);
    });

    response_json.in_progress = in_progress_total;
    response_json.completed = completed_total;

    Ok(response_json)
}
