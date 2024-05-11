use serde::{Deserialize, Serialize};

use super::{ assets::Assets, codex::{CodexResponse, StringOrI32}, potentials::Potentials, summary::EquipItem };
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct NftListResponse {
    pub data: NftDataObject,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NftDataObject {
    pub lists: Vec<Nft>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Nft {
    pub seq: i32,
    #[serde(alias = "transportID")]
    pub transport_id: i32,
    #[serde(alias = "nftID")]
    pub nft_id: String,
    #[serde(alias = "sealedDT")]
    pub sealed_dt: i32,
    #[serde(alias = "characterName")]
    pub character_name: String,
    pub class: i32,
    #[serde(alias = "lv")]
    pub lvl: i32,
    #[serde(alias = "powerScore")]
    pub power_score: i32,
    pub price: i32,
    #[serde(alias = "MirageScore")]
    pub mirage_score: i32,
    #[serde(alias = "MiraX")]
    pub mira_x: i32,
    #[serde(alias = "Reinforce")]
    pub reinforce: i32,
    #[serde(default)]
    pub world_name: String,
    #[serde(default)]
    pub trade_type: i32,
    #[serde(default)]
    pub stats: HashMap<String, f32>,
    #[serde(default)]
    pub skills: HashMap<String, i32>,
    #[serde(default)]
    pub training: HashMap<String, StringOrI32>,
    #[serde(default)]
    pub buildings: HashMap<String, i32>,
    #[serde(default)]
    pub assets: Assets,
    #[serde(default)]
    pub potentials: Potentials,
    #[serde(default)]
    pub holy_stuff: HashMap<String, i32>,
    #[serde(default)]
    pub codex: CodexResponse,
    #[serde(default)]
    pub equip_items: HashMap<String, EquipItem>,
    #[serde(default)]
    pub inventory_id: i64,
    #[serde(default)]
    pub succession_id: i64,
    #[serde(default)]
    pub spirits_id: i64,
    #[serde(default)]
    pub magic_orb_id: i64,
    #[serde(default)]
    pub magic_stone_id: i64,
    #[serde(default)]
    pub mystical_piece_id: i64,
}
