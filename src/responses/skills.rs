use reqwest_middleware::ClientWithMiddleware;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::utils::get_response;

#[derive(Serialize, Deserialize, Debug)]
pub struct SkillsResponse {
    pub data: Vec<SkillObject>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SkillObject {
    #[serde(alias = "skillLevel")]
    pub skill_level: String,
    #[serde(alias = "skillName")]
    pub skill_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Skills {
    pub skills: HashMap<String, String>,
}

pub async fn get_nft_skills(
    transport_id: i32,
    character_class: i32,
    client: ClientWithMiddleware,
) -> anyhow::Result<HashMap<String, String>> {
    let request_url = format!(
        "https://webapi.mir4global.com/nft/character/skills?transportID={transport_id}&class={character_class}&languageCode=en",
        transport_id = transport_id,
        character_class = character_class
    );

    let response_json: SkillsResponse = get_response(&client, request_url).await?;

    let skills_hashmap: HashMap<String, String> = response_json
        .data
        .iter()
        .map(|skill_object| {
            (
                skill_object.skill_name.clone(),
                skill_object.skill_level.clone(),
            )
        })
        .collect();

    Ok(skills_hashmap)
}
