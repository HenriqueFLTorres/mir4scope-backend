use mongodb::bson::doc;
use serde::{ Deserialize, Serialize };
use std::collections::HashMap;

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
    pub consitution_level: u8,
    #[serde(alias = "consitutionName")]
    pub consitution_name: String,
    #[serde(alias = "collectName")]
    pub collect_name: String,
    #[serde(alias = "collectLevel")]
    pub collect_level: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TrainingObject {
    #[serde(alias = "forceIdx")]
    pub force_idx: String,
    #[serde(alias = "forceLevel")]
    pub force_level: String,
    #[serde(alias = "forceName")]
    pub force_name: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct Training {
    pub training: HashMap<String, String>,
    pub constitution: u8,
    #[serde(alias = "collectName")]
    pub collect_name: String,
    #[serde(alias = "collectLevel")]
    pub collect_level: u8,
}

pub async fn get_nft_training(
    transport_id: serde_json::Value,
    client: reqwest::Client
) -> anyhow::Result<Training> {
    let request_url = format!(
        "https://webapi.mir4global.com/nft/character/training?transportID={transport_id}&languageCode=en",
        transport_id = transport_id
    );

    let response = client.get(request_url).send().await?.text().await?;

    let response_json: TrainingResponse = serde_json::from_str(&response).unwrap();
    let training_hashmap: HashMap<String, String> = HashMap::from([
        ("Violet Mist Art".to_string(), response_json.data.violet_mist_art.force_level),
        (
            "Muscle Strength Manual".to_string(),
            response_json.data.muscle_strength_manual.force_level,
        ),
        ("Nine Yang Manual".to_string(), response_json.data.nine_yang_manual.force_level),
        ("Toad Stance".to_string(), response_json.data.toad_stance.force_level),
        ("Northern Profound Art".to_string(), response_json.data.northern_profound_art.force_level),
        ("Nine Yin Manual".to_string(), response_json.data.nine_yin_manual.force_level),
    ]);

    let training_to_db: Training = Training {
        training: training_hashmap,
        collect_level: response_json.data.collect_level,
        collect_name: response_json.data.collect_name,
        constitution: response_json.data.consitution_level,
    };

    Ok(training_to_db)
}
