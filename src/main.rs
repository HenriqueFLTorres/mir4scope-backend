use std::env;
use std::sync::Arc;
use std::time::Instant;
use responses::nft::NftListResponse;
use tokio::sync::Mutex;
use tokio::task::JoinError;

use mongodb::bson;
use mongodb::{
    bson::doc,
    options::{ ClientOptions, FindOneOptions, ResolverConfig },
    Client,
    Collection,
    Database,
};
use utils::State;

use crate::responses::codex::get_nft_codex;
use crate::responses::{
    assets::get_nft_assets,
    building::get_nft_buildings,
    holy_stuff::get_nft_holy_stuff,
    inventory::get_nft_inventory,
    magic_orb::get_nft_magic_orb,
    magic_stone::get_nft_magic_stone,
    mystical_piece::get_nft_mystical_piece,
    nft::Nft,
    potentials::get_nft_potentials,
    skills::get_nft_skills,
    spirits::get_nft_spirits,
    stats::get_nft_stats,
    succession::get_nft_succession,
    summary::get_nft_summary,
    training::get_nft_training,
};

mod responses;
mod utils;

const DATABASE_NAME: &str = "Mir4Scope";
static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().expect(".env file not found");

    let subscriber = tracing_subscriber
        ::fmt()
        .pretty()
        .with_max_level(tracing::Level::ERROR)
        .finish();
    tracing::subscriber
        ::set_global_default(subscriber)
        .expect("Can't set default tracing subscriber");

    // Load the MongoDB connection string from an environment variable:
    let client_uri = env
        ::var("MONGODB_URI")
        .expect("You must set the MONGODB_URI environment var!");

    // A Client is needed to connect to MongoDB:
    // An extra line of code to work around a DNS issue on Windows:
    let options = ClientOptions::parse_with_resolver_config(
        &client_uri,
        ResolverConfig::cloudflare()
    ).await?;
    let mongodb_client = Client::with_options(options)?;
    let database = mongodb_client.database(DATABASE_NAME);

    let binding = Arc::new(
        Mutex::new(State {
            nft_collection: database.collection("nft"),
            database,
            client: reqwest::Client::builder().user_agent(APP_USER_AGENT).build()?,
        })
    );

    let state = binding.lock().await;

    let now = Instant::now();

    let _ = tokio::join!(
        tokio::spawn(
            retrieve_and_save_nft(
                state.nft_collection.to_owned(),
                state.database.to_owned(),
                state.client.to_owned(),
                1
            )
        )
    );

    let elapsed = now.elapsed();
    tracing::info!("retrieve_and_save_nft function time: {:#?}", elapsed);

    Ok(())
}

async fn retrieve_and_save_nft(
    nft_collection: Collection<Nft>,
    database: Database,
    client: reqwest::Client,
    page_index: i32
) -> anyhow::Result<()> {
    let request_url = format!(
        "https://webapi.mir4global.com/nft/lists?listType=sale&class=0&levMin=0&levMax=0&powerMin=0&powerMax=0&priceMin=0&priceMax=0&sort=latest&page={page}&languageCode=en",
        page = page_index
    );

    let response = client.get(request_url).send().await?;
    let resonse_json: NftListResponse = response.json().await?;

    let nft_list = serde_json::to_value(resonse_json.data.lists)?;

    let list = match nft_list {
        serde_json::Value::Array(arr) => arr,
        _ => Vec::new(),
    };

    let opts = FindOneOptions::builder().skip(2).build();
    let tasks: Vec<_> = list
        .into_iter()
        .map(|character|
            tokio::spawn(
                dump_nft(
                    character,
                    opts.clone(),
                    nft_collection.clone(),
                    database.clone(),
                    client.clone()
                )
            )
        )
        .collect();

    for task in tasks {
        match task.await.unwrap() {
            Ok(f) => f,
            Err(error) => tracing::error!("Error dumping nft: {:#?}", error),
        };
    }

    Ok(())
}

async fn dump_nft(
    nft_data: serde_json::Value,
    opts: FindOneOptions,
    nft_collection: Collection<Nft>,
    database: Database,
    client: reqwest::Client
) -> Result<(), JoinError> {
    let mut character: Nft = serde_json::from_value(nft_data.clone()).unwrap();
    let record = nft_collection
        .find_one(Some(doc! { "seq": bson::to_bson(&character.seq).unwrap() }), opts.clone()).await
        .unwrap();

    if record.is_some() {
        println!(
            "End of nft dumper, a match was found in the db with the name of {}!",
            character.character_name
        );
    }

    println!("Dumping character with the name of {}...", character.character_name);

    let (stats, skills, training, buildings, assets, potentials, holy_stuff, codex) =
        tokio::join!(
            tokio::spawn(get_nft_stats(character.transport_id, client.clone())),
            tokio::spawn(get_nft_skills(character.transport_id, character.class, client.clone())),
            tokio::spawn(get_nft_training(character.transport_id, client.clone())),
            tokio::spawn(get_nft_buildings(character.transport_id, client.clone())),
            tokio::spawn(get_nft_assets(character.transport_id, client.clone())),
            tokio::spawn(get_nft_potentials(character.transport_id, client.clone())),
            tokio::spawn(get_nft_holy_stuff(character.transport_id, client.clone())),
            tokio::spawn(get_nft_codex(character.transport_id, client.clone()))
        );

    match (stats, skills, training, buildings, assets, potentials, holy_stuff, codex) {
        (
            Ok(stats),
            Ok(skills),
            Ok(training),
            Ok(buildings),
            Ok(assets),
            Ok(potentials),
            Ok(holy_stuff),
            Ok(codex),
        ) => {
            character.stats = stats.unwrap();
            character.skills = skills.unwrap();
            character.training = training.unwrap();
            character.buildings = buildings.unwrap();
            character.assets = assets.unwrap();
            character.potentials = potentials.unwrap();
            character.holy_stuff = holy_stuff.unwrap();
            character.codex = codex.unwrap();
        }
        | (Err(err), _, _, _, _, _, _, _)
        | (_, Err(err), _, _, _, _, _, _)
        | (_, _, Err(err), _, _, _, _, _)
        | (_, _, _, Err(err), _, _, _, _)
        | (_, _, _, _, Err(err), _, _, _)
        | (_, _, _, _, _, Err(err), _, _)
        | (_, _, _, _, _, _, Err(err), _)
        | (_, _, _, _, _, _, _, Err(err)) =>
            tracing::error!("Error joining nft_creation auxiliary tasks {:#?}", err),
    }
    nft_collection
        .insert_one(
            bson::from_document::<Nft>(bson::to_document(&character).unwrap()).unwrap(),
            None
        ).await
        .unwrap();

    let _ = tokio
        ::spawn(
            get_nft_spirits(
                nft_collection.clone(),
                character.transport_id,
                client.clone(),
                database.clone()
            )
        ).await
        .unwrap();
    let _ = tokio
        ::spawn(
            get_nft_magic_orb(
                nft_collection.clone(),
                character.transport_id,
                client.clone(),
                database.clone()
            )
        ).await
        .unwrap();

    let nft_inventory = tokio::join!(
        tokio::spawn(
            get_nft_inventory(
                nft_collection.clone(),
                character.transport_id,
                client.clone(),
                database.clone()
            )
        )
    );

    match nft_inventory {
        (Ok(nft_inv),) => {
            let inv = Arc::new(nft_inv.unwrap());

            let _ = tokio
                ::spawn(
                    get_nft_summary(
                        nft_collection.clone(),
                        character.seq,
                        character.transport_id,
                        character.class,
                        client.clone(),
                        inv.clone().to_vec()
                    )
                ).await
                .unwrap();
            let _ = tokio
                ::spawn(
                    get_nft_magic_stone(
                        nft_collection.clone(),
                        character.transport_id,
                        character.class,
                        client.clone(),
                        database.clone(),
                        inv.clone().to_vec()
                    )
                ).await
                .unwrap();
            let _ = tokio
                ::spawn(
                    get_nft_mystical_piece(
                        nft_collection.clone(),
                        character.transport_id,
                        character.class,
                        client.clone(),
                        database.clone(),
                        inv.clone().to_vec()
                    )
                ).await
                .expect("Failed to get mystical piece");
            let _ = tokio
                ::spawn(
                    get_nft_succession(
                        nft_collection.clone(),
                        character.transport_id,
                        character.class,
                        client.clone(),
                        database.clone(),
                        inv.clone().to_vec()
                    )
                ).await
                .unwrap();

            Ok(())
        }
        (Err(err),) => {
            tracing::error!(
                "Error joining task `nft_inventory` (seq: {:#?}, transport_id: {:#?}, name: {:#?}): {:#?}",
                character.seq,
                character.transport_id,
                character.character_name,
                err
            );
            Ok(())
        }
    }
}
