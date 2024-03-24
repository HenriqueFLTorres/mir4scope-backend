use crate::utils::object_id;
use serde::{ Deserialize, Deserializer, de, Serialize };
use serde_json::Value;
use crate::Nft;
use mongodb::{ bson, bson::doc, Collection };

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct AssetsResponse {
    pub data: Assets,
    #[serde(alias = "nftID")]
    #[serde(default = "object_id")]
    pub nft_id: mongodb::bson::oid::ObjectId,
}

fn to_string<'de, D: Deserializer<'de>>(deserializer: D) -> Result<String, D::Error> {
    Ok(match Value::deserialize(deserializer)? {
        Value::String(s) => s.to_string(),
        Value::Number(num) => num.to_string(),
        _ => {
            return Err(de::Error::custom("wrong type"));
        }
    })
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct Assets {
    pub copper: String,
    pub energy: String,
    pub darksteel: String,
    pub speedups: String,
    #[serde(deserialize_with = "to_string")]
    pub dragonjade: String,
    pub acientcoins: String,
    #[serde(deserialize_with = "to_string")]
    pub dragonsteel: String,
}

pub async fn get_nft_assets(
    nft_collection: &Collection<Nft>,
    transport_id: &serde_json::Value,
    client: &reqwest::Client
) -> anyhow::Result<()> {
    let request_url = format!(
        "https://webapi.mir4global.com/nft/character/assets?transportID={transport_id}&languageCode=en",
        transport_id = transport_id
    );

    let response = client.get(request_url).send().await?.text().await?;
    let response_json: AssetsResponse = serde_json::from_str(&response)?;

    let filter = doc! { "transport_id": bson::to_bson(transport_id)? };
    let update = doc! { "$set": { "assets": bson::to_bson(&response_json.data)?} };

    nft_collection.update_one(filter, update, None).await?;

    Ok(())
}
