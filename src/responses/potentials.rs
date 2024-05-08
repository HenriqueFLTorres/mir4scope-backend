use reqwest_middleware::ClientWithMiddleware;
use serde::{Deserialize, Serialize};

use crate::utils::get_response;

#[derive(Serialize, Deserialize, Debug)]
pub struct PotentialsResponse {
    pub data: Potentials,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Potentials {
    total: i32,
    #[serde(alias = "totalMax")]
    total_max: i32,
    hunting: i32,
    #[serde(alias = "huntingMax")]
    hunting_max: i32,
    pvp: i32,
    #[serde(alias = "pvpMax")]
    pvp_max: i32,
    secondary: i32,
    #[serde(alias = "secondaryMax")]
    secondary_max: i32,
}

pub async fn get_nft_potentials(
    transport_id: i32,
    client: ClientWithMiddleware,
) -> anyhow::Result<Potentials> {
    let request_url = format!(
        "https://webapi.mir4global.com/nft/character/potential?transportID={transport_id}&languageCode=en",
        transport_id = transport_id
    );

    let response_json: PotentialsResponse = get_response(&client, request_url).await?;

    Ok(response_json.data)
}
