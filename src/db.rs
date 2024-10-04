use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;
use crate::responses::magic_orb::MagicOrbResponse;
use crate::responses::magic_stone::MagicStoneResponseObject;
use crate::responses::mystical_piece::MysticalPieceResponseObject;
use crate::responses::spirits::SpiritsObject;
use crate::responses::{inventory::InventoryResponse, nft::Nft, succession::SuccessionResponse};

pub async fn create_pool() -> Result<Pool<Postgres>, sqlx::Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}

pub async fn add_nft(pool: &Pool<Postgres>, character: &Nft) -> Result<(), sqlx::Error> {
    let query = r#"
      INSERT INTO nft (character_name, seq, transport_id, nft_id, sealed_dt, class, lvl, power_score, price, mirage_score, mira_x, reinforce, trade_type, world_name, stats, skills, training, buildings, assets, potentials, holy_stuff, codex, equip_items, tickets, inventory_id, succession_id, spirits_id, magic_orb_id, magic_stone_id, mystical_piece_id)
      VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30)
    "#;

    let stats_json_string = serde_json::to_string(&character.stats).unwrap();

    sqlx::query(query)
        .bind(character.character_name.clone())
        .bind(character.seq)
        .bind(character.transport_id)
        .bind(character.nft_id.clone())
        .bind(character.sealed_dt)
        .bind(character.class)
        .bind(character.lvl)
        .bind(character.power_score)
        .bind(character.price)
        .bind(character.mirage_score)
        .bind(character.mira_x)
        .bind(character.reinforce)
        .bind(character.trade_type)
        .bind(character.world_name.clone())
        .bind(serde_json::from_str::<serde_json::Value>(stats_json_string.as_str()).unwrap())
        .bind(serde_json::json!(&character.skills))
        .bind(serde_json::json!(&character.training))
        .bind(serde_json::json!(&character.buildings))
        .bind(serde_json::json!(&character.assets))
        .bind(serde_json::json!(&character.potentials))
        .bind(serde_json::json!(&character.holy_stuff))
        .bind(serde_json::json!(&character.codex))
        .bind(serde_json::json!(&character.equip_items))
        .bind(serde_json::json!(&character.tickets))
        .bind(character.inventory_id)
        .bind(character.succession_id)
        .bind(character.spirits_id)
        .bind(character.magic_orb_id)
        .bind(character.magic_stone_id)
        .bind(character.mystical_piece_id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn add_inventory(
    pool: &Pool<Postgres>,
    inventory_response: &InventoryResponse,
) -> Result<i64, sqlx::Error> {
    let query = r#"
      INSERT INTO inventory (inventory, craft_materials)
      VALUES ($1, $2)
      RETURNING id
    "#;

    let json_items = serde_json::json!(&inventory_response.inventory);
    let json_craft_materials = serde_json::json!(&inventory_response.craft_materials);

    let id: (i64,) = sqlx::query_as(query)
        .bind(json_items)
        .bind(json_craft_materials)
        .fetch_one(pool)
        .await?;

    Ok(id.0)
}

pub async fn add_succession(
    pool: Pool<Postgres>,
    succession_response: &SuccessionResponse,
) -> anyhow::Result<i64> {
    let query = r#"
        INSERT INTO succession (succession)
        VALUES ($1)
        RETURNING id
    "#;

    let json_items = serde_json::json!(&succession_response.data.equip_item);

    let id: (i64,) = sqlx::query_as(query)
        .bind(json_items)
        .fetch_one(&pool)
        .await?;

    Ok(id.0)
}

pub async fn add_spirits(
    pool: Pool<Postgres>,
    spirits_response: &SpiritsObject,
) -> anyhow::Result<i64> {
    let query = r#"
        INSERT INTO spirits (equip, inven)
        VALUES ($1, $2)
        RETURNING id
    "#;

    let equip = serde_json::json!(&spirits_response.equip);
    let inven = serde_json::json!(&spirits_response.inven);

    let id: (i64,) = sqlx::query_as(query)
        .bind(equip)
        .bind(inven)
        .fetch_one(&pool)
        .await?;

    Ok(id.0)
}

pub async fn add_magic_orb(
    pool: Pool<Postgres>,
    magic_orb_response: &MagicOrbResponse,
) -> anyhow::Result<i64> {
    let query = r#"
        INSERT INTO magic_orb (equip_item, active_deck)
        VALUES ($1, $2)
        RETURNING id
    "#;

    let equip_item = serde_json::json!(&magic_orb_response.data.equip_item);

    let id: (i64,) = sqlx::query_as(query)
        .bind(equip_item)
        .bind(magic_orb_response.data.active_deck)
        .fetch_one(&pool)
        .await?;

    Ok(id.0)
}
pub async fn add_magic_stone(
    pool: Pool<Postgres>,
    magic_stone_response: &MagicStoneResponseObject,
) -> anyhow::Result<i64> {
    let query = r#"
        INSERT INTO magic_stone (equip_item, active_deck)
        VALUES ($1, $2)
        RETURNING id
    "#;

    let equip_item = serde_json::json!(&magic_stone_response.equip_item);

    let id: (i64,) = sqlx::query_as(query)
        .bind(equip_item)
        .bind(magic_stone_response.active_deck)
        .fetch_one(&pool)
        .await?;

    Ok(id.0)
}

pub async fn add_mystical_piece(
    pool: Pool<Postgres>,
    mystical_piece_response: &MysticalPieceResponseObject,
) -> anyhow::Result<i64> {
    let query = r#"
        INSERT INTO mystical_piece (equip_item, active_deck)
        VALUES ($1, $2)
        RETURNING id
    "#;

    let equip_item = serde_json::json!(&mystical_piece_response.equip_item);

    let id: (i64,) = sqlx::query_as(query)
        .bind(equip_item)
        .bind(mystical_piece_response.active_deck)
        .fetch_one(&pool)
        .await?;

    Ok(id.0)
}
