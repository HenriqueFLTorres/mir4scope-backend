use serde::{ Deserialize, Serialize, Serializer };
use std::collections::HashMap;
use crate::Nft;
use mongodb::{ bson, bson::doc, Collection };

#[derive(Serialize, Deserialize, Debug)]
pub struct HolyStuffResponse {
    pub data: HashMap<String, HolyStuffObject>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HolyStuffObject {
    #[serde(alias = "HolyStuffName")]
    pub holy_stuff_name: String,
    #[serde(alias = "Grade", serialize_with = "serialize_grade_value")]
    pub grade: String,
}

fn serialize_grade_value<S>(grade: &String, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer
{
    if grade.is_empty() { serializer.serialize_str("0") } else { serializer.serialize_str(grade) }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct HolyStuff {
    pub holy_stuff: HashMap<String, String>,
}

pub async fn get_nft_holy_stuff(
    nft_collection: &Collection<Nft>,
    transport_id: &serde_json::Value,
    client: &reqwest::Client
) -> anyhow::Result<()> {
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

    let filter = doc! { "transport_id": bson::to_bson(transport_id)? };
    let update = doc! { "$set": { "holy_stuff": bson::to_bson(&holy_stuff_hashmap)?} };

    nft_collection.update_one(filter, update, None).await?;

    Ok(())
}
