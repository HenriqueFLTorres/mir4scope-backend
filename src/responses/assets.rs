use reqwest_middleware::ClientWithMiddleware;
use serde::{de, Deserialize, Deserializer, Serialize};
use serde_json::Value;

use crate::utils::get_response;

#[derive(Serialize, Deserialize, Debug)]
pub struct AssetsResponse {
    pub data: Assets,
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

#[derive(Serialize, Deserialize, Debug, Default)]
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
    transport_id: i32,
    client: ClientWithMiddleware,
) -> anyhow::Result<Assets> {
    let request_url = format!(
        "https://webapi.mir4global.com/nft/character/assets?transportID={transport_id}&languageCode=en",
        transport_id = transport_id
    );

    let response_json: AssetsResponse = get_response(&client, request_url).await?;

    Ok(response_json.data)
}
