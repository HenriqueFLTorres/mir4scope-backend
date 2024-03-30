use mongodb::bson::doc;
use serde::{Deserialize, Serialize, Serializer};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct ItemDetailResponse {
    pub data: ItemDetailData,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct ItemDetailData {
    #[serde(alias = "powerScore")]
    pub power_score: u32,
    pub options: Vec<ItemDetail>,
    #[serde(alias = "addOptions", default)]
    pub add_option: Vec<ItemDetailAdd>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct ItemDetail {
    #[serde(alias = "optionName")]
    pub name: String,
    #[serde(alias = "optionValue", serialize_with = "serialize_float_rounded")]
    pub value: f64,
    #[serde(alias = "optionFormat")]
    pub format: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
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
    client: &reqwest::Client,
    transport_id: &u32,
    class: &u32,
    item_uid: &String,
) -> anyhow::Result<ItemDetailData> {
    let request_url = format!(
        "https://webapi.mir4global.com/nft/character/itemdetail?transportID={transport_id}&class={class}&itemUID={item_uid}&languageCode=en",
        transport_id = transport_id,
        class = class,
        item_uid = item_uid
    );

    let response = client.get(&request_url).send().await?.text().await?;
    let response_json: ItemDetailResponse = serde_json::from_str(&response)?;

    Ok(response_json.data)
}
