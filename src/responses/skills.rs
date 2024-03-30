use mongodb::bson::doc;
use serde::{ Deserialize, Serialize };
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct SkillsResponse {
    pub data: Vec<SkillObject>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct SkillObject {
    #[serde(alias = "skillLevel")]
    pub skill_level: String,
    #[serde(alias = "skillName")]
    pub skill_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct Skills {
    pub skills: HashMap<String, String>,
}

pub async fn get_nft_skills(
    transport_id: u32,
    character_class: u32,
    client: reqwest::Client
) -> anyhow::Result<HashMap<String, String>> {
    let request_url = format!(
        "https://webapi.mir4global.com/nft/character/skills?transportID={transport_id}&class={character_class}&languageCode=en",
        transport_id = transport_id,
        character_class = character_class
    );

    let response = client.get(request_url).send().await?.text().await?;

    let response_json: SkillsResponse = serde_json::from_str(&response)?;
    let skills_hashmap: HashMap<String, String> = response_json.data
        .iter()
        .map(|skill_object| { (skill_object.skill_name.clone(), skill_object.skill_level.clone()) })
        .collect();

    Ok(skills_hashmap)
}
