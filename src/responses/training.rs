use reqwest_middleware::ClientWithMiddleware;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::utils::get_response;

use super::codex::StringOrI32;

#[derive(Serialize, Deserialize, Debug)]
pub struct TrainingResponse {
    pub code: u16,
    pub data: TrainingResponseData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TrainingResponseData {
    #[serde(alias = "0")]
    pub muscle_strength_manual: TrainingObject,
    #[serde(alias = "1")]
    pub nine_yin_manual: TrainingObject,
    #[serde(alias = "2")]
    pub nine_yang_manual: TrainingObject,
    #[serde(alias = "3")]
    pub violet_mist_art: TrainingObject,
    #[serde(alias = "4")]
    pub northern_profound_art: TrainingObject,
    #[serde(alias = "5")]
    pub toad_stance: TrainingObject,
    #[serde(alias = "consitutionLevel")]
    pub consitution_level: StringOrI32,
    #[serde(alias = "consitutionName")]
    pub consitution_name: StringOrI32,
    #[serde(alias = "collectName")]
    pub collect_name: StringOrI32,
    #[serde(alias = "collectLevel")]
    pub collect_level: StringOrI32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TrainingObject {
    #[serde(alias = "forceIdx")]
    pub force_idx: String,
    #[serde(alias = "forceLevel")]
    pub force_level: StringOrI32,
    #[serde(alias = "forceName")]
    pub force_name: String,
}

pub async fn get_nft_training(
    transport_id: i32,
    client: ClientWithMiddleware
) -> anyhow::Result<HashMap<String, StringOrI32>> {
    let request_url = format!(
        "https://webapi.mir4global.com/nft/character/training?transportID={transport_id}&languageCode=en",
        transport_id = transport_id
    );

    let response_json: TrainingResponse = get_response(&client, request_url).await?;
    let data = response_json.data;

    let training_hashmap: HashMap<String, StringOrI32> = HashMap::from([
        (
            "Violet Mist Art".to_string(),
            StringOrI32::Integer(data.violet_mist_art.force_level.as_i32().unwrap()),
        ),
        (
            "Muscle Strength Manual".to_string(),
            StringOrI32::Integer(data.muscle_strength_manual.force_level.as_i32().unwrap()),
        ),
        (
            "Nine Yang Manual".to_string(),
            StringOrI32::Integer(data.nine_yang_manual.force_level.as_i32().unwrap()),
        ),
        (
            "Toad Stance".to_string(),
            StringOrI32::Integer(data.toad_stance.force_level.as_i32().unwrap()),
        ),
        (
            "Northern Profound Art".to_string(),
            StringOrI32::Integer(data.northern_profound_art.force_level.as_i32().unwrap()),
        ),
        (
            "Nine Yin Manual".to_string(),
            StringOrI32::Integer(data.nine_yin_manual.force_level.as_i32().unwrap()),
        ),
        ("Constitution".to_string(), data.consitution_level),
        ("collect_name".to_string(), data.collect_name),
        ("collect_level".to_string(), data.collect_level),
    ]);

    Ok(training_hashmap)
}
