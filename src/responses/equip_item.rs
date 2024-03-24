use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct EquipItem {
    #[serde(alias = "itemIdx")]
    pub item_idx: String,
    pub enhance: String,
    #[serde(alias = "refineStep")]
    pub refine_step: String,
    pub grade: String,
    pub tier: String,
    #[serde(alias = "itemType")]
    pub item_type: String,
    #[serde(alias = "itemName")]
    pub item_name: String,
    #[serde(alias = "itemPath")]
    pub item_path: String,
}
