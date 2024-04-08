use std::collections::HashMap;

use mongodb::bson::doc;
use serde::{ Deserialize, Deserializer, Serialize, Serializer };

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct CodexResponse {
    pub data: HashMap<String, Codex>,
    #[serde(deserialize_with = "string_to_u32", serialize_with = "u32_to_string")]
    #[serde(skip_deserializing)]
    #[serde(default)]
    pub completed_codex: u32
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct Codex {
    #[serde(alias = "codexName")]
    pub codex_name: String,
    #[serde(alias = "totalCount")]
    #[serde(deserialize_with = "string_to_u32", serialize_with = "u32_to_string")]
    pub total_count: u32,
    #[serde(deserialize_with = "string_to_u32", serialize_with = "u32_to_string")]
    pub completed: u32,
    #[serde(alias = "inprogress")]
    #[serde(deserialize_with = "string_to_u32", serialize_with = "u32_to_string")]
    pub in_progress: u32,
}

pub fn string_to_u32<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.parse::<u32>().map_err(serde::de::Error::custom)
}

pub fn u32_to_string<S>(value: &u32, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&value.to_string())
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

    let mut x: Vec<u32> = Vec::new();
    let c = response_json.data.iter().for_each(|i| {
        x.push(i.1.completed);
    });

    let mut sum: u32 = 0;
    x.into_iter().for_each(|num| {
        sum += num
    });

    println!("{sum:#?}");
    response_json.completed_codex = sum;

    Ok(response_json)
}
