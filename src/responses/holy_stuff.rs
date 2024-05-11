use reqwest_middleware::ClientWithMiddleware;
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

use crate::utils::get_response;

#[derive(Serialize, Deserialize, Debug)]
pub struct HolyStuffResponse {
    pub data: HashMap<String, HolyStuffObject>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HolyStuffObject {
    #[serde(alias = "HolyStuffName")]
    pub holy_stuff_name: String,
    #[serde(alias = "Grade", deserialize_with = "parse_grade_value")]
    pub grade: String,
}

fn parse_grade_value<'de, D>(d: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    Deserialize::deserialize(d).map(|x: Option<_>| x.unwrap_or("0".to_string()))
}
#[derive(Serialize, Deserialize, Debug)]
pub struct HolyStuff {
    pub holy_stuff: HashMap<String, String>,
}

pub async fn get_nft_holy_stuff(
    transport_id: i32,
    client: ClientWithMiddleware,
) -> anyhow::Result<HashMap<String, i32>> {
    let request_url = format!(
        "https://webapi.mir4global.com/nft/character/holystuff?transportID={transport_id}&languageCode=en",
        transport_id = transport_id
    );

    let response_json: HolyStuffResponse = get_response(&client, request_url).await?;

    let holy_stuff_hashmap: HashMap<String, i32> = response_json
        .data
        .iter()
        .map(|holy_stuff_object| {
            let value_as_number = holy_stuff_object.1.grade.parse::<i32>().unwrap();

            (
                holy_stuff_object.1.holy_stuff_name.clone(),
                value_as_number.clone(),
            )
        })
        .collect();

    Ok(holy_stuff_hashmap)
}
