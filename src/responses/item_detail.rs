use reqwest_middleware::ClientWithMiddleware;
use serde::{Deserialize, Serialize, Serializer};

use crate::utils::get_response;

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemDetailResponse {
    pub data: ItemDetailData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemDetailData {
    #[serde(alias = "powerScore")]
    pub power_score: i32,
    pub options: Vec<ItemDetail>,
    #[serde(alias = "addOptions", default)]
    pub add_option: Vec<ItemDetailAdd>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemDetail {
    #[serde(alias = "optionName")]
    pub name: String,
    #[serde(alias = "optionValue", serialize_with = "serialize_float_rounded")]
    pub value: f64,
    #[serde(alias = "optionFormat")]
    pub format: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemDetailAdd {
    #[serde(alias = "optionName")]
    pub name: String,
    #[serde(alias = "optionValue", serialize_with = "serialize_float_rounded")]
    pub value: f64,
    #[serde(alias = "optionAddFormat")]
    pub format: String,
}

fn serialize_float_rounded<S>(value: &f64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let decimal_places = 2;
    let rounded_value =
        (value * (10_f64).powi(decimal_places)).round() / (10_f64).powi(decimal_places);
    serializer.serialize_f64(rounded_value)
}

pub async fn get_item_detail(
    client: &ClientWithMiddleware,
    transport_id: &i32,
    class: &i32,
    item_uid: &String,
) -> anyhow::Result<ItemDetailData> {
    let request_url = format!(
        "https://webapi.mir4global.com/nft/character/itemdetail?transportID={transport_id}&class={class}&itemUID={item_uid}&languageCode=en",
        transport_id = transport_id,
        class = class,
        item_uid = item_uid
    );

    let response_json: ItemDetailResponse = get_response(client, request_url).await?;

    Ok(response_json.data)
}
