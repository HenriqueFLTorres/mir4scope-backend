use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;

use mongodb::bson;
use mongodb::bson::oid::ObjectId;
use mongodb::{
    bson::doc,
    options::{ClientOptions, FindOneOptions, ResolverConfig},
    Client, Collection, Database,
};

use responses::{
    nft::Nft,
    stats::StatsResponse,
    skills::{ SkillsResponse, Skills }
};

mod utils;
mod responses;

const DATABASE_NAME: &str = "Mir4Scope";
static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().expect(".env file not found");

    let subscriber = tracing_subscriber::fmt().finish();
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

    let mir4scope_database = mongodb_client.database(DATABASE_NAME);
    let nft_collection: Collection<Nft> = mir4scope_database.collection("Nft");

    retrieve_and_save_nft(nft_collection, mir4scope_database).await?;

    Ok(())
}

async fn retrieve_and_save_nft(
    nft_collection: Collection<Nft>,
    database: Database,
) -> anyhow::Result<()> {
    let client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()?;

    let request_url =
        "https://webapi.mir4global.com/nft/lists?listType=sale&class=0&levMin=0&levMax=0&powerMin=0&powerMax=0&priceMin=0&priceMax=0&sort=latest&page=1&languageCode=en";

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

                get_nft_summary(nft_collection.clone(), &character["seq"], client.clone())
                    .await
                    .expect("can't get nft summary");
                get_nft_stats(
                    nft_collection.clone(),
                    &character["transportID"],
                    client.clone(),
                    database.clone(),
                    record.inserted_id.as_object_id().unwrap(),
                )
                .await
                .expect("can't get nft stats");
                get_nft_skills(
                    nft_collection.clone(),
                    &character["transportID"],
                    &character["class"],
                    client.clone(),
                    database.clone(),
                    record.inserted_id.as_object_id().unwrap()
                )
                .await
                .expect("can't get nft skills");
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
    client: reqwest::Client,
) -> anyhow::Result<()> {
    let request_url = format!(
        "https://webapi.mir4global.com/nft/character/summary?seq={seq}&languageCode=en",
        seq = seq
    );

    let response = client.get(request_url).send().await?;
    let json: serde_json::Value = response.json().await?;
    let data = &json["data"];

    let filter = doc! { "seq": bson::to_bson(seq)? };
    let update = doc! { "$set": { "trade_type": bson::to_bson(&data["tradeType"])?, "world_name": bson::to_bson(&data["character"]["worldName"])?, "equip_items": bson::to_bson(&data["equipItem"])? } };

    nft_collection.update_one(filter, update, None).await?;

    Ok(())
}

async fn get_nft_stats(
    nft_collection: Collection<Nft>,
    transport_id: &serde_json::Value,
    client: reqwest::Client,
    database: Database,
    record_id: ObjectId,
) -> anyhow::Result<()> {
    let request_url = format!(
        "https://webapi.mir4global.com/nft/character/stats?transportID={transport_id}&languageCode=en",
        transport_id = transport_id
    );

    let response = client.get(request_url).send().await?.text().await?;

    let mut stats_json: StatsResponse = serde_json::from_str(&response)?;
    stats_json.nft_id = record_id;

    println!("{:#?}", stats_json);

    let stats_collection = database.collection("Stats");

    let record = stats_collection.insert_one(stats_json, None).await?;
    let filter = doc! { "transport_id": bson::to_bson(transport_id)? };
    let update = doc! { "$set": { "stats_id": record.inserted_id.as_object_id()  } };

    nft_collection.update_one(filter, update, None).await?;

    Ok(())
}

async fn get_nft_skills(
    nft_collection: Collection<Nft>,
    transport_id: &serde_json::Value,
    character_class: &serde_json::Value,
    client: reqwest::Client,
    database: Database,
    record_id: ObjectId,
) -> anyhow::Result<()> {
    let request_url = format!(
        "https://webapi.mir4global.com/nft/character/skills?transportID={transport_id}&class={character_class}&languageCode=en",
        transport_id = transport_id,
        character_class = character_class,
    );

    let response = client.get(request_url).send().await?.text().await?;

    let response_json: SkillsResponse = serde_json::from_str(&response)?;
    let skills_hashmap: HashMap<String, String> = response_json.data.iter().map(|skill_object| (skill_object.skill_name.clone(), skill_object.skill_level.clone())).collect();

    let skills_to_db: Skills = Skills { skills: skills_hashmap, nft_id: record_id };

    let skills_collection = database.collection("Skills");

    let record = skills_collection.insert_one(skills_to_db, None).await?;
    let filter = doc! { "transport_id": bson::to_bson(transport_id)? };
    let update = doc! { "$set": { "skills_id": record.inserted_id.as_object_id()  } };

    nft_collection.update_one(filter, update, None).await?;

    Ok(())
}
