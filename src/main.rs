use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use responses::nft::{Nft, NftListResponse};
use sqlx::{Pool, Postgres};
use std::fs;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;
use tokio::task::{JoinError, JoinSet};

use crate::responses::magic_orb::get_nft_magic_orb;
use crate::responses::magic_stone::get_nft_magic_stone;
use crate::responses::mystical_piece::get_nft_mystical_piece;
use crate::responses::spirits::get_nft_spirits;
use crate::responses::succession::get_nft_succession;
use crate::responses::ticket::get_nft_tickets;
use crate::responses::{
    assets::get_nft_assets, building::get_nft_buildings, codex::get_nft_codex,
    holy_stuff::get_nft_holy_stuff, inventory::get_nft_inventory, potentials::get_nft_potentials,
    skills::get_nft_skills, stats::get_nft_stats, summary::get_nft_summary,
    training::get_nft_training,
};
use crate::utils::AppState;
use crate::utils::{get_response, nft_description_error};

mod db;
mod responses;
mod utils;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().expect(".env file not found");

    let subscriber = tracing_subscriber::fmt().pretty().finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Can't set default tracing subscriber");

    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(1);
    let basic_client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    let bindings = Arc::new(Mutex::new(AppState {
        db: db::create_pool().await?,
        client: ClientBuilder::new(basic_client)
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
            .build(),
    }));
    let app_state = bindings.lock().await;

    let now = Instant::now();

    let data = fs::read_to_string("src/dump_trade_items/list.json").unwrap();
    // let data = fs::read_to_string("list.json").unwrap();
    let traddable_list: serde_json::Value =
        serde_json::from_str(&data).expect("list.json file was not found");

    let delete_all_queries = vec![
        "DELETE FROM nft",
        "DELETE FROM inventory",
        "DELETE FROM succession",
        "DELETE FROM spirits",
        "DELETE FROM magic_orb",
        "DELETE FROM magic_stone",
        "DELETE FROM mystical_piece",
    ];

    for query in delete_all_queries {
        sqlx::query(query)
            .execute(&app_state.db.to_owned())
            .await
            .unwrap();
    }

    let mut join_set = JoinSet::new();
    for i in 1..3 {
        join_set.spawn(retrieve_and_save_nft(
            app_state.client.to_owned(),
            i,
            app_state.db.to_owned(),
            traddable_list.clone(),
        ));
    }
    while let Some(res) = join_set.join_next().await {
        let out = res?;
        match out {
            Err(err) => tracing::error!("Error spawning NFT list: {}", err),
            Ok(_) => {}
        }
    }

    let elapsed = now.elapsed();
    tracing::info!("retrieve_and_save_nft function time: {:#?}", elapsed);

    Ok(())
}

async fn retrieve_and_save_nft(
    client: ClientWithMiddleware,
    page_index: i32,
    database: Pool<Postgres>,
    traddable_list: serde_json::Value,
) -> anyhow::Result<()> {
    let request_url = format!(
        "https://webapi.mir4global.com/nft/lists?listType=sale&class=0&levMin=0&levMax=0&powerMin=0&powerMax=0&priceMin=0&priceMax=0&sort=latest&page={page}&languageCode=en",
        page = page_index
    );

    let response_json: NftListResponse = get_response(&client, request_url).await.unwrap();

    let nft_list = serde_json::to_value(response_json.data.lists)?;

    let list = match nft_list {
        serde_json::Value::Array(arr) => arr,
        _ => Vec::new(),
    };

    let tasks: Vec<_> = list
        .into_iter()
        .map(|character| {
            tokio::spawn(dump_nft(
                character,
                database.clone(),
                client.clone(),
                traddable_list.clone(),
            ))
        })
        .collect();

    for task in tasks {
        task.await
            .unwrap()
            .unwrap_or_else(|error| tracing::error!("Error dumping nft: {:#?}", error));
    }

    Ok(())
}

async fn dump_nft(
    nft_data: serde_json::Value,
    pool: Pool<Postgres>,
    client: ClientWithMiddleware,
    traddable_list: serde_json::Value,
) -> anyhow::Result<(), JoinError> {
    let mut character: Nft = serde_json::from_value(nft_data.clone()).expect(
        &nft_description_error("Fail to get serde_json value from nft", nft_data.clone()),
    );

    let db_transport_id: (bool,) = sqlx::query_as(
        "
      select
          exists (
            select
              1
            from
              nft
            where
              transport_id = $1
      );",
    )
    .bind(character.transport_id)
    .fetch_one(&pool)
    .await
    .unwrap();

    if db_transport_id.0 {
        tracing::info!(
            "transport_id: {} exist in the database",
            character.transport_id
        );
        return Ok(());
    }

    let nft_inventory = tokio::spawn(get_nft_inventory(character.transport_id, client.clone()))
        .await
        .expect(&nft_description_error(
            "Fail to get nft inventory",
            nft_data.clone(),
        ))
        .unwrap();

    let succession = tokio::spawn(get_nft_succession(
        character.transport_id,
        client.clone(),
        character.class,
        nft_inventory.clone().inventory,
    ))
    .await
    .expect(&nft_description_error(
        "Fail to get nft succession",
        nft_data.clone(),
    ))
    .unwrap();
    let spirits = tokio::spawn(get_nft_spirits(character.transport_id, client.clone()))
        .await
        .expect(&nft_description_error(
            "Fail to get nft spirits",
            nft_data.clone(),
        ))
        .unwrap();
    let magic_orb = tokio::spawn(get_nft_magic_orb(character.transport_id, client.clone()))
        .await
        .expect(&nft_description_error(
            "Fail to get nft magic_orb",
            nft_data.clone(),
        ))
        .unwrap();
    let magic_stone = tokio::spawn(get_nft_magic_stone(
        character.transport_id,
        character.class,
        client.clone(),
        nft_inventory.clone().inventory,
    ))
    .await
    .expect(&nft_description_error(
        "Fail to get nft magic_stone",
        nft_data.clone(),
    ))
    .unwrap();
    let mystical_piece = tokio::spawn(get_nft_mystical_piece(
        character.transport_id,
        character.class,
        client.clone(),
        nft_inventory.clone().inventory,
    ))
    .await
    .expect(&nft_description_error(
        "Fail to get nft mystical_piece",
        nft_data.clone(),
    ))
     .unwrap();
    
    character.tickets = tokio::spawn(get_nft_tickets(
        nft_inventory.clone().inventory,
    ))
    .await
    .expect(&nft_description_error(
        "Fail to get nft tickets",
        nft_data.clone(),
    ))
    .unwrap();

    let (summary, stats, skills, training, buildings, assets, potentials, holy_stuff, codex) = tokio::join!(
        tokio::spawn(get_nft_summary(
            character.seq,
            character.transport_id,
            character.class,
            client.clone(),
            nft_inventory.clone().inventory,
            traddable_list.clone()
        )),
        tokio::spawn(get_nft_stats(character.transport_id, client.clone())),
        tokio::spawn(get_nft_skills(
            character.transport_id,
            character.class,
            client.clone()
        )),
        tokio::spawn(get_nft_training(character.transport_id, client.clone())),
        tokio::spawn(get_nft_buildings(character.transport_id, client.clone())),
        tokio::spawn(get_nft_assets(character.transport_id, client.clone())),
        tokio::spawn(get_nft_potentials(character.transport_id, client.clone())),
        tokio::spawn(get_nft_holy_stuff(character.transport_id, client.clone())),
        tokio::spawn(get_nft_codex(character.transport_id, client.clone()))
    );

    match (
        summary, stats, skills, training, buildings, assets, potentials, holy_stuff, codex,
    ) {
        (
            Ok(summary),
            ..,
            Ok(stats),
            Ok(skills),
            Ok(training),
            Ok(buildings),
            Ok(assets),
            Ok(potentials),
            Ok(holy_stuff),
            Ok(codex),
        ) => {
            let summary_data = summary.unwrap();
            character.trade_type = summary_data.trade_type;
            character.world_name = summary_data.world_name;
            character.equip_items = summary_data.equip_items;

            character.stats = stats.unwrap();
            character.skills = skills.unwrap();
            character.training = training.unwrap();
            character.buildings = buildings.unwrap();
            character.assets = assets.unwrap();
            character.potentials = potentials.unwrap();
            character.holy_stuff = holy_stuff.unwrap();
            character.codex = codex.unwrap();
        }
        (Err(err), _, _, _, _, _, _, _, _)
        | (_, Err(err), _, _, _, _, _, _, _)
        | (_, _, Err(err), _, _, _, _, _, _)
        | (_, _, _, Err(err), _, _, _, _, _)
        | (_, _, _, _, Err(err), _, _, _, _)
        | (_, _, _, _, _, Err(err), _, _, _)
        | (_, _, _, _, _, _, Err(err), _, _)
        | (_, _, _, _, _, _, _, Err(err), _)
        | (_, _, _, _, _, _, _, _, Err(err)) => {
            tracing::error!("Error joining nft_creation auxiliary tasks {:#?}", err);
        }
    }

    println!(
        "Dumping character with the name of {:#?}...",
        character.character_name
    );

    character.inventory_id = db::add_inventory(&pool.clone(), &nft_inventory)
        .await
        .unwrap();
    character.succession_id = db::add_succession(pool.clone(), &succession).await.unwrap();
    character.spirits_id = db::add_spirits(pool.clone(), &spirits).await.unwrap();
    character.magic_orb_id = db::add_magic_orb(pool.clone(), &magic_orb).await.unwrap();
    character.magic_stone_id = db::add_magic_stone(pool.clone(), &magic_stone)
        .await
        .unwrap();
    character.mystical_piece_id = db::add_mystical_piece(pool.clone(), &mystical_piece)
        .await
        .unwrap();

    db::add_nft(&pool.clone().clone(), &character)
        .await
        .expect(&nft_description_error(
            "Fail to add nft to database",
            nft_data.clone(),
        ));

    Ok(())
}
