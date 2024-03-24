use mongodb::bson::doc;
use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct Nft {
    pub seq: u32,
    #[serde(alias = "transportID")]
    pub transport_id: u32,
    #[serde(alias = "nftID")]
    pub nft_id: String,
    #[serde(alias = "sealedDT")]
    pub sealed_dt: u32,
    #[serde(alias = "characterName")]
    pub character_name: String,
    pub class: u32,
    #[serde(alias = "lv")]
    pub lvl: u32,
    #[serde(alias = "powerScore")]
    pub power_score: u32,
    pub price: u32,
    #[serde(alias = "MirageScore")]
    pub mirage_score: u32,
    #[serde(alias = "MiraX")]
    pub mira_x: u32,
    #[serde(alias = "Reinforce")]
    pub reinforce: u32,
}
