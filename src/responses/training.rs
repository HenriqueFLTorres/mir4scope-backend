use crate::utils::object_id;
use serde::{Deserialize, Serialize};
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct Training {
    pub training: HashMap<String, String>,
    pub constitution: u8,
    #[serde(alias = "collectName")]
    pub collect_name: String,
    #[serde(alias = "collectLevel")]
    pub collect_level: u8,
}
