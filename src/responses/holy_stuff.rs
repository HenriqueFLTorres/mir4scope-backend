use mongodb::bson::doc;
use serde::{ Deserialize, Deserializer, Serialize };
use std::collections::HashMap;

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

fn parse_grade_value<'de, D>(d: D) -> Result<String, D::Error> where D: Deserializer<'de> {
    Deserialize::deserialize(d).map(|x: Option<_>| { x.unwrap_or("0".to_string()) })
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct HolyStuff {
    pub holy_stuff: HashMap<String, String>,
}

pub async fn get_nft_holy_stuff(
    transport_id: u32,
    client: reqwest::Client
) -> anyhow::Result<HashMap<String, String>> {
    let request_url = format!(
        "https://webapi.mir4global.com/nft/character/holystuff?transportID={transport_id}&languageCode=en",
        transport_id = transport_id
    );

    let response = client.get(request_url).send().await?.text().await?;
    let response_json: HolyStuffResponse = serde_json::from_str(&response)?;
    let holy_stuff_hashmap: HashMap<String, String> = response_json.data
        .iter()
        .map(|holy_stuff_object| {
            (holy_stuff_object.1.holy_stuff_name.clone(), holy_stuff_object.1.grade.clone())
        })
        .collect();

    Ok(holy_stuff_hashmap)
}
