extern crate dotenv;

use mongodb::{
    Client,
    bson::{ doc },
    options::{ FindOneOptions, ClientOptions, ResolverConfig },
    Collection,
};
use mongodb::bson;
use dotenv::dotenv;
use serde::{ Deserialize, Serialize };
use serde_json::{ Result, Map, Value };
use std::env;
use tokio;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use convert_case::{ Case, Casing };

mod nft;
use nft::{ Nft, Summary, EquipItem, Character };

const DATABASE_NAME: &str = "Mir4Scope";
static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

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

    let mir4scope_database = mongodb_client.database(DATABASE_NAME);
    let nft_collection: Collection<Nft> = mir4scope_database.collection("Nft");

    retrieve_and_save_nft(nft_collection).await;

    Ok(())
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

async fn retrieve_and_save_nft(
    nft_collection: Collection<Nft>
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder().user_agent(APP_USER_AGENT).build()?;

    let request_url =
        "https://webapi.mir4global.com/nft/lists?listType=sale&class=0&levMin=0&levMax=0&powerMin=0&powerMax=0&priceMin=0&priceMax=0&sort=latest&page=1&languageCode=en";

    let response = client.get(request_url).send().await?;
    let users: serde_json::Value = response.json().await?;

    let opts = FindOneOptions::builder().skip(2).build();

    if let serde_json::Value::Array(nft_list) = &users["data"]["lists"] {
        for character in nft_list {
            let record = nft_collection.find_one(
                Some(doc! { "seq": bson::to_bson(&character["seq"]).unwrap() }),
                opts.clone()
            ).await?;

            if let Some(_) = record {
                println!(
                    "End of nft dumper, a match was found in the db with the name of {}!",
                    character["characterName"]
                );
                break;
            } else {
                println!("Dumping character with the name of {}...", character["characterName"]);
                let nft_data: Nft = serde_json::from_value(character.clone()).unwrap();
                let nft_document = bson::to_document(&nft_data)?;

                nft_collection.insert_one(
                    bson::from_document::<Nft>(nft_document).unwrap(),
                    None
                ).await?;
                get_nft_summary(nft_collection.clone(), &character["seq"], client.clone()).await;
            }
        }
    }

    let mut file = File::create("output.json")?;
    let json_string = serde_json::to_string_pretty(&users["data"]["lists"])?;
    file.write_all(json_string.as_bytes())?;

    Ok(())
}

async fn get_nft_summary(
    nft_collection: Collection<Nft>,
    seq: &serde_json::Value,
    client: reqwest::Client
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let request_url = format!(
        "https://webapi.mir4global.com/nft/character/summary?seq={seq}&languageCode=en",
        seq = seq
    );

    let response = client.get(request_url).send().await?;
    let json: serde_json::Value = response.json().await?;
    let data = &json["data"];

    let summary_data: Summary = serde_json::from_value(data.clone()).unwrap();

    let filter = doc! { "seq": bson::to_bson(seq).unwrap() };
    let update =
        doc! { "$set": { "trade_type": bson::to_bson(&data["tradeType"]).unwrap(), "world_name": bson::to_bson(&data["character"]["worldName"]).unwrap(), "equip_items": bson::to_bson(&data["equipItem"]).unwrap() } };

    nft_collection.update_one(filter, update, None).await.unwrap();

    Ok(())
}
