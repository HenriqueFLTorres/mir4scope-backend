use reqwest_middleware::ClientWithMiddleware;
use serde::{ Deserialize, Serialize };
use std::collections::HashMap;
use regex::Regex;

use crate::utils::get_response;

#[derive(Serialize, Deserialize, Debug)]
pub struct StatsResponse {
    pub data: StatsObject,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatsObject {
    pub lists: Vec<Stats>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Stats {
    #[serde(alias = "statName")]
    pub stat_name: String,
    #[serde(alias = "statValue")]
    pub stat_value: String,
    #[serde(alias = "iconPath")]
    pub icon_path: String,
}

pub async fn get_nft_stats(
    transport_id: i32,
    client: ClientWithMiddleware
) -> anyhow::Result<HashMap<String, f32>> {
    let request_url = format!(
        "https://webapi.mir4global.com/nft/character/stats?transportID={transport_id}&languageCode=en",
        transport_id = transport_id
    );

    let response_json: StatsResponse = get_response(&client, request_url).await?;

    let re = Regex::new(r"%|,|sec").unwrap();
    let stats_hashmap: HashMap<String, f32> = response_json.data.lists
        .iter()
        .map(|stats_object| {
            let parsed_value = re.replace_all(stats_object.stat_value.as_str(), "");
            let value_as_number = parsed_value.into_owned().parse::<f32>().unwrap();

            (stats_object.stat_name.clone(), value_as_number.clone())
        })
        .collect();

    Ok(stats_hashmap)
}
