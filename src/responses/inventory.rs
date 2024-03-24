use serde::{ Deserialize, Serialize };

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
