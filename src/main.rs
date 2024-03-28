use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;

use mongodb::bson;
use mongodb::{
    bson::doc,
    options::{ClientOptions, FindOneOptions, ResolverConfig},
    Client, Collection, Database,
};
use utils::State;

use crate::responses::{
    assets::get_nft_assets, building::get_nft_buildings, holy_stuff::get_nft_holy_stuff,
    inventory::get_nft_inventory, magic_orb::get_nft_magic_orb, magic_stone::get_nft_magic_stone,
    mystical_piece::get_nft_mystical_piece, nft::Nft, potentials::get_nft_potentials,
    skills::get_nft_skills, spirits::get_nft_spirits, stats::get_nft_stats,
    succession::get_nft_succession, summary::get_nft_summary, training::get_nft_training,
};

mod responses;
mod utils;

const DATABASE_NAME: &str = "Mir4Scope";
static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().expect(".env file not found");

    let subscriber = tracing_subscriber::fmt()
        .pretty()
        .with_max_level(tracing::Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Can't set default tracing subscriber");

    // Load the MongoDB connection string from an environment variable:
    let client_uri =
        env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

    // A Client is needed to connect to MongoDB:
    // An extra line of code to work around a DNS issue on Windows:
    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await?;
    let mongodb_client = Client::with_options(options)?;
    let database = mongodb_client.database(DATABASE_NAME);

    let binding = Arc::new(Mutex::new(State {
        nft_collection: database.collection("Nft"),
        database,
        client: reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .build()?,
    }));

    let state = binding.lock().await;

    let now = Instant::now();

    retrieve_and_save_nft(
        state.nft_collection.to_owned(),
        state.database.to_owned(),
        state.client.to_owned(),
    )
    .await?;

    let elapsed = now.elapsed();
    tracing::info!("retrieve_and_save_nft function time: {:#?}", elapsed);

    Ok(())
}

async fn retrieve_and_save_nft(
    nft_collection: Collection<Nft>,
    database: Database,
    client: reqwest::Client,
) -> anyhow::Result<()> {
    let request_url = format!(
        "https://webapi.mir4global.com/nft/lists?listType=sale&class=0&levMin=0&levMax=0&powerMin=0&powerMax=0&priceMin=0&priceMax=0&sort=latest&page={page}&languageCode=en",
        page = 1
    );

    let response = client.get(request_url).send().await?;
    let users: serde_json::Value = response.json().await?;

    let opts = FindOneOptions::builder().skip(2).build();

    if let serde_json::Value::Array(nft_list) = &users["data"]["lists"] {
        for character in nft_list {
            let record = nft_collection
                .find_one(
                    Some(doc! { "seq": bson::to_bson(&character["seq"])? }),
                    opts.clone(),
                )
                .await?;

            if record.is_some() {
                println!(
                    "End of nft dumper, a match was found in the db with the name of {}!",
                    character["characterName"]
                );
                break;
            } else {
                println!(
                    "Dumping character with the name of {}...",
                    character["characterName"]
                );
                let nft_data: Nft = serde_json::from_value(character.clone())?;
                let nft_document = bson::to_document(&nft_data)?;

                let record = nft_collection
                    .insert_one(bson::from_document::<Nft>(nft_document)?, None)
                    .await?;

                tokio::spawn(get_nft_stats(
                    nft_collection.clone(),
                    character["transportID"].clone(),
                    client.clone(),
                    database.clone(),
                ));
                tokio::spawn(get_nft_skills(
                    nft_collection.clone(),
                    character["transportID"].clone(),
                    character["class"].clone(),
                    client.clone(),
                    database.clone(),
                ));
                tokio::spawn(get_nft_spirits(
                    nft_collection.clone(),
                    character["seq"].clone(),
                    client.clone(),
                    database.clone(),
                ));
                tokio::spawn(get_nft_training(
                    nft_collection.clone(),
                    character["transportID"].clone(),
                    client.clone(),
                ));
                tokio::spawn(get_nft_buildings(
                    nft_collection.clone(),
                    character["transportID"].clone(),
                    client.clone(),
                ));
                tokio::spawn(get_nft_assets(
                    nft_collection.clone(),
                    character["transportID"].clone(),
                    client.clone(),
                ));
                tokio::spawn(get_nft_potentials(
                    nft_collection.clone(),
                    character["transportID"].clone(),
                    client.clone(),
                ));
                tokio::spawn(get_nft_holy_stuff(
                    nft_collection.clone(),
                    character["transportID"].clone(),
                    client.clone(),
                ));
                tokio::spawn(get_nft_succession(
                    nft_collection.clone(),
                    character["transportID"].clone(),
                    client.clone(),
                ));
                tokio::spawn(get_nft_magic_orb(
                    nft_collection.clone(),
                    character["transportID"].clone(),
                    client.clone(),
                    database.clone(),
                ));

                let nft_inventory = tokio::join!(tokio::spawn(get_nft_inventory(
                    nft_collection.clone(),
                    character["transportID"].clone(),
                    client.clone(),
                    database.clone(),
                    record.inserted_id.as_object_id().unwrap(),
                )));

                match nft_inventory {
                    (Ok(nft_inv),) => {
                        let inv = Arc::new(nft_inv.unwrap());

                        tokio::spawn(get_nft_summary(
                            nft_collection.clone(),
                            character["seq"].clone(),
                            character["transportID"].clone(),
                            character["class"].clone(),
                            client.clone(),
                            inv.clone().to_vec(),
                        ));
                        tokio::spawn(get_nft_magic_stone(
                            nft_collection.clone(),
                            character["transportID"].clone(),
                            character["class"].clone(),
                            client.clone(),
                            database.clone(),
                            inv.clone().to_vec(),
                        ));
                        tokio::spawn(get_nft_mystical_piece(
                            nft_collection.clone(),
                            character["transportID"].clone(),
                            character["class"].clone(),
                            client.clone(),
                            database.clone(),
                            inv.clone().to_vec(),
                        ));
                    }
                    (Err(err),) => tracing::error!("Error joining task `nft_inventory` {:#?}", err),
                }
            }
        }
        tokio::time::sleep(std::time::Duration::from_secs(60)).await; // wait to all running tasks
    }

    let mut file = File::create("output.json")?;
    let json_string = serde_json::to_string_pretty(&users["data"]["lists"])?;
    file.write_all(json_string.as_bytes())?;

    Ok(())
}
