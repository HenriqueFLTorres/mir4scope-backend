use serde::{ Deserialize, Serialize };
use crate::Nft;
use mongodb::{ bson, bson::doc, Collection, Database };

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct InventoryResponse {
    #[serde(alias = "data")]
    pub inventory: Vec<InventoryItem>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct InventoryItem {
    #[serde(alias = "itemUID")]
    pub item_uid: String,
    #[serde(alias = "itemID")]
    pub item_id: String,
    pub enhance: u8,
    pub stack: u32,
    #[serde(alias = "tranceStep")]
    pub trance_step: u8,
    #[serde(alias = "RefineStep")]
    pub refine_step: u8,
    pub grade: String,
    #[serde(alias = "mainType")]
    pub main_type: u8,
    #[serde(alias = "subType")]
    pub sub_type: u8,
    #[serde(alias = "tabCategory")]
    pub tab_category: u8,
    pub tier: String,
    #[serde(alias = "itemName")]
    pub item_name: String,
    #[serde(alias = "itemPath")]
    pub item_path: String,
}

pub async fn get_nft_inventory(
    nft_collection: &Collection<Nft>,
    transport_id: &serde_json::Value,
    client: &reqwest::Client,
    database: &Database
) -> anyhow::Result<()> {
    let request_url = format!(
        "https://webapi.mir4global.com/nft/character/inven?transportID={transport_id}&languageCode=en",
        transport_id = transport_id
    );

    let response = client.get(request_url).send().await?.text().await?;
    let response_json: InventoryResponse = serde_json::from_str(&response)?;

    let inventory_collection = database.collection("Inventory");

    let record = inventory_collection.insert_one(response_json, None).await?;
    let filter = doc! { "transport_id": bson::to_bson(transport_id)? };
    let update = doc! { "$set": { "inventory_id": record.inserted_id.as_object_id() } };

    nft_collection.update_one(filter, update, None).await?;

    Ok(())
}
