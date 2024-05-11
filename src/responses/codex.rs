use std::collections::HashMap;

use reqwest_middleware::ClientWithMiddleware;
use serde::{Deserialize, Serialize};

use crate::{responses, utils::get_response};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CodexResponse {
    pub data: HashMap<String, Codex>,
    #[serde(default)]
    pub in_progress: i32,
    #[serde(default)]
    pub completed: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Codex {
    #[serde(alias = "codexName")]
    pub codex_name: String,
    #[serde(alias = "totalCount")]
    pub total_count: StringOrI32,
    pub completed: StringOrI32,
    #[serde(alias = "inprogress")]
    pub in_progress: StringOrI32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum StringOrI32 {
    String(String),
    Integer(i32),
}

impl StringOrI32 {
    pub fn as_i32(&self) -> Option<i32> {
        match self {
            StringOrI32::Integer(num) => Some(*num),
            StringOrI32::String(s) => s.parse().ok(),
        }
    }
}

pub async fn get_nft_codex(
    transport_id: i32,
    client: ClientWithMiddleware,
) -> anyhow::Result<CodexResponse> {
    let request_url = format!(
        "https://webapi.mir4global.com/nft/character/codex?transportID={transport_id}&languageCode=en",
        transport_id = transport_id
    );

    let mut response_json: CodexResponse = get_response(&client, request_url).await?;

    let mut in_progress_total = 0;
    let mut completed_total = 0;
    response_json.data.iter_mut().for_each(|(_, codex)| {
        let in_progress_int: i32 = codex.in_progress.as_i32().unwrap();
        let completed_int: i32 = codex.completed.as_i32().unwrap();
        let total_count_int: i32 = codex.total_count.as_i32().unwrap();

        in_progress_total += in_progress_int;
        completed_total += completed_int;
        codex.in_progress = responses::codex::StringOrI32::Integer(in_progress_int);
        codex.completed = responses::codex::StringOrI32::Integer(completed_int);
        codex.total_count = responses::codex::StringOrI32::Integer(total_count_int);
    });

    response_json.in_progress = in_progress_total;
    response_json.completed = completed_total;

    Ok(response_json)
}
