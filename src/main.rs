use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;

use mongodb::bson;
use mongodb::{
    bson::doc,
    options::{ClientOptions, FindOneOptions, ResolverConfig},
    Client, Collection, Database,
};

use crate::responses::assets::AssetsResponse;
use crate::responses::building::{Building, BuildingResponse};
use crate::responses::holy_stuff::HolyStuffResponse;
use crate::responses::potentials::PotentialsResponse;
use crate::responses::training::{Training, TrainingResponse};
use crate::responses::succession::SuccessionResponse;
use crate::responses::magic_orb::MagicOrbResponse;
use crate::responses::magic_stone::MagicStoneResponse;
use responses::{nft::Nft, skills::SkillsResponse, spirits::SpiritsResponse, stats::StatsResponse};

mod responses;
mod utils;

const DATABASE_NAME: &str = "Mir4Scope";
static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().expect(".env file not found");

    let subscriber = tracing_subscriber::fmt()
        .pretty()
        .with_max_level(tracing::Level::DEBUG)
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

                let _record = nft_collection
                    .insert_one(bson::from_document::<Nft>(nft_document)?, None)
                    .await?;
                let _ = tokio::try_join!(
                    get_nft_summary(&nft_collection, &character["seq"], &client),
                    get_nft_stats(
                        &nft_collection,
                        &character["transportID"],
                        &client,
                        &database
                    ),
                    get_nft_skills(
                        &nft_collection,
                        &character["transportID"],
                        &character["class"],
                        &client,
                        &database
                    ),
                    get_nft_spirits(
                        &nft_collection,
                        &character["transportID"],
                        &client,
                        &database
                    ),
                    get_nft_training(&nft_collection, &character["transportID"], &client),
                    get_nft_buildings(&nft_collection, &character["transportID"], &client),
                    get_nft_assets(&nft_collection, &character["transportID"], &client),
                    get_nft_potentials(&nft_collection, &character["transportID"], &client),
                    get_nft_holy_stuff(&nft_collection, &character["transportID"], &client),
                    get_nft_succession(&nft_collection, &character["transportID"], &client),
                    get_nft_magic_orb(
                        &nft_collection,
                        &character["transportID"],
                        &client,
                        &database
                    ),
                    get_nft_magic_stone(
                        &nft_collection,
                        &character["transportID"],
                        &client,
                        &database
                    )
                );
            }
        }
    }

    let mut file = File::create("output.json")?;
    let json_string = serde_json::to_string_pretty(&users["data"]["lists"])?;
    file.write_all(json_string.as_bytes())?;

    Ok(())
}

async fn get_nft_summary(
    nft_collection: &Collection<Nft>,
    seq: &serde_json::Value,
    client: &reqwest::Client,
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
    nft_collection: &Collection<Nft>,
    transport_id: &serde_json::Value,
    client: &reqwest::Client,
    database: &Database,
) -> anyhow::Result<()> {
    let request_url = format!(
        "https://webapi.mir4global.com/nft/character/stats?transportID={transport_id}&languageCode=en",
        transport_id = transport_id
    );

    let response = client.get(request_url).send().await?.text().await?;

    let stats_json: StatsResponse = serde_json::from_str(&response)?;

    let stats_collection = database.collection("Stats");

    let record = stats_collection.insert_one(stats_json, None).await?;
    let filter = doc! { "transport_id": bson::to_bson(transport_id)? };
    let update = doc! { "$set": { "stats_id": record.inserted_id.as_object_id()  } };

    nft_collection.update_one(filter, update, None).await?;

    Ok(())
}

async fn get_nft_skills(
    nft_collection: &Collection<Nft>,
    transport_id: &serde_json::Value,
    character_class: &serde_json::Value,
    client: &reqwest::Client,
    database: &Database,
) -> anyhow::Result<()> {
    let request_url = format!(
        "https://webapi.mir4global.com/nft/character/skills?transportID={transport_id}&class={character_class}&languageCode=en",
        transport_id = transport_id,
        character_class = character_class,
    );

    let response = client.get(request_url).send().await?.text().await?;

    let response_json: SkillsResponse = serde_json::from_str(&response)?;
    let skills_hashmap: HashMap<String, String> = response_json
        .data
        .iter()
        .map(|skill_object| {
            (
                skill_object.skill_name.clone(),
                skill_object.skill_level.clone(),
            )
        })
        .collect();

    let skills_collection = database.collection("Skills");

    let record = skills_collection.insert_one(skills_hashmap, None).await?;
    let filter = doc! { "transport_id": bson::to_bson(transport_id)? };
    let update = doc! { "$set": { "skills_id": record.inserted_id.as_object_id()  } };

    nft_collection.update_one(filter, update, None).await?;

    Ok(())
}

async fn get_nft_training(
    nft_collection: &Collection<Nft>,
    transport_id: &serde_json::Value,
    client: &reqwest::Client,
) -> anyhow::Result<()> {
    let request_url = format!(
        "https://webapi.mir4global.com/nft/character/training?transportID={transport_id}&languageCode=en",
        transport_id = transport_id,
    );

    let response = client.get(request_url).send().await?.text().await?;

    let response_json: TrainingResponse = serde_json::from_str(&response).unwrap();
    let training_hashmap: HashMap<String, String> = HashMap::from([
        (
            "Violet Mist Art".to_string(),
            response_json.data.violet_mist_art.force_level,
        ),
        (
            "Muscle Strength Manual".to_string(),
            response_json.data.muscle_strength_manual.force_level,
        ),
        (
            "Nine Yang Manual".to_string(),
            response_json.data.nine_yang_manual.force_level,
        ),
        (
            "Toad Stance".to_string(),
            response_json.data.toad_stance.force_level,
        ),
        (
            "Northern Profound Art".to_string(),
            response_json.data.northern_profound_art.force_level,
        ),
        (
            "Nine Yin Manual".to_string(),
            response_json.data.nine_yin_manual.force_level,
        ),
    ]);

    let training_to_db: Training = Training {
        training: training_hashmap,
        collect_level: response_json.data.collect_level,
        collect_name: response_json.data.collect_name,
        constitution: response_json.data.consitution_level,
    };

    let filter = doc! { "transport_id": bson::to_bson(transport_id)? };
    let update = doc! { "$set": bson::to_bson(&training_to_db)? };

    nft_collection.update_one(filter, update, None).await?;

    Ok(())
}

async fn get_nft_buildings(
    nft_collection: &Collection<Nft>,
    transport_id: &serde_json::Value,
    client: &reqwest::Client,
) -> anyhow::Result<()> {
    let request_url = format!(
        "https://webapi.mir4global.com/nft/character/building?transportID={transport_id}&languageCode=en",
        transport_id = transport_id,
    );

    let response = client.get(request_url).send().await?.text().await?;

    let response_json: BuildingResponse = serde_json::from_str(&response).unwrap();
    let building_hashmap: HashMap<String, String> = response_json
        .data
        .iter()
        .map(|building_object| {
            (
                building_object.1.building_name.clone(),
                building_object.1.building_level.clone(),
            )
        })
        .collect();

    let building_to_db: Building = Building {
        building: building_hashmap,
    };

    let filter = doc! { "transport_id": bson::to_bson(transport_id)? };
    let update = doc! { "$set": bson::to_bson(&building_to_db)? };

    nft_collection.update_one(filter, update, None).await?;

    Ok(())
}

async fn get_nft_spirits(
    nft_collection: &Collection<Nft>,
    transport_id: &serde_json::Value,
    client: &reqwest::Client,
    database: &Database,
) -> anyhow::Result<()> {
    let request_url = format!(
        "https://webapi.mir4global.com/nft/character/spirit?transportID={transport_id}&languageCode=en",
        transport_id = transport_id,
    );

    let response = client.get(request_url).send().await?.text().await?;
    let response_json: SpiritsResponse = serde_json::from_str(&response)?;

    let spirits_collection = database.collection("Spirits");

    let record = spirits_collection.insert_one(response_json, None).await?;
    let filter = doc! { "transport_id": bson::to_bson(transport_id)? };
    let update = doc! { "$set": { "spirits_id": record.inserted_id.as_object_id() } };

    nft_collection.update_one(filter, update, None).await?;

    Ok(())
}

async fn get_nft_assets(
    nft_collection: &Collection<Nft>,
    transport_id: &serde_json::Value,
    client: &reqwest::Client,
) -> anyhow::Result<()> {
    let request_url = format!(
        "https://webapi.mir4global.com/nft/character/assets?transportID={transport_id}&languageCode=en",
        transport_id = transport_id,
    );

    let response = client.get(request_url).send().await?.text().await?;
    let response_json: AssetsResponse = serde_json::from_str(&response)?;

    let filter = doc! { "transport_id": bson::to_bson(transport_id)? };
    let update = doc! { "$set": { "assets": bson::to_bson(&response_json.data)?}  };

    nft_collection.update_one(filter, update, None).await?;

    Ok(())
}

async fn get_nft_potentials(
    nft_collection: &Collection<Nft>,
    transport_id: &serde_json::Value,
    client: &reqwest::Client,
) -> anyhow::Result<()> {
    let request_url = format!(
        "https://webapi.mir4global.com/nft/character/potential?transportID={transport_id}&languageCode=en",
        transport_id = transport_id,
    );

    let response = client.get(request_url).send().await?.text().await?;
    let response_json: PotentialsResponse = serde_json::from_str(&response)?;

    let filter = doc! { "transport_id": bson::to_bson(transport_id)? };
    let update = doc! { "$set": { "potentials": bson::to_bson(&response_json.data)?}  };

    nft_collection.update_one(filter, update, None).await?;

    Ok(())
}

async fn get_nft_holy_stuff(
    nft_collection: &Collection<Nft>,
    transport_id: &serde_json::Value,
    client: &reqwest::Client,
) -> anyhow::Result<()> {
    let request_url = format!(
        "https://webapi.mir4global.com/nft/character/holystuff?transportID={transport_id}&languageCode=en",
        transport_id = transport_id,
    );

    let response = client.get(request_url).send().await?.text().await?;
    let response_json: HolyStuffResponse = serde_json::from_str(&response)?;
    let holy_stuff_hashmap: HashMap<String, String> = response_json
        .data
        .iter()
        .map(|holy_stuff_object| {
            (
                holy_stuff_object.1.holy_stuff_name.clone(),
                holy_stuff_object.1.grade.clone(),
            )
        })
        .collect();

    let filter = doc! { "transport_id": bson::to_bson(transport_id)? };
    let update = doc! { "$set": { "holy_stuff": bson::to_bson(&holy_stuff_hashmap)?}  };

    nft_collection.update_one(filter, update, None).await?;

    Ok(())
}

async fn get_nft_succession(
    nft_collection: &Collection<Nft>,
    transport_id: &serde_json::Value,
    client: &reqwest::Client,
) -> anyhow::Result<()> {
    let request_url = format!(
        "https://webapi.mir4global.com/nft/character/succession?transportID={transport_id}&languageCode=en",
        transport_id = transport_id
    );

    let response = client.get(request_url).send().await?.text().await?;
    let response_json: SuccessionResponse = serde_json::from_str(&response)?;

    let filter = doc! { "transport_id": bson::to_bson(transport_id)? };
    let update = doc! { "$set": { "succession": bson::to_bson(&response_json.data.equip_item)? }  };

    nft_collection.update_one(filter, update, None).await?;

    Ok(())
}

async fn get_nft_magic_orb(
    nft_collection: &Collection<Nft>,
    transport_id: &serde_json::Value,
    client: &reqwest::Client,
    database: &Database,
) -> anyhow::Result<()> {
    let request_url = format!(
        "https://webapi.mir4global.com/nft/character/magicorb?transportID={transport_id}&languageCode=en",
        transport_id = transport_id,
    );

    let response = client.get(request_url).send().await?.text().await?;
    let response_json: MagicOrbResponse = serde_json::from_str(&response)?;

    let magic_orb_collection = database.collection("Magic Orb");

    let record = magic_orb_collection.insert_one(response_json.data, None).await?;
    let filter = doc! { "transport_id": bson::to_bson(transport_id)? };
    let update = doc! { "$set": { "magic_orb_id": record.inserted_id.as_object_id() } };

    nft_collection.update_one(filter, update, None).await?;

    Ok(())
}

async fn get_nft_magic_stone(
    nft_collection: &Collection<Nft>,
    transport_id: &serde_json::Value,
    client: &reqwest::Client,
    database: &Database,
) -> anyhow::Result<()> {
    let request_url = format!(
        "https://webapi.mir4global.com/nft/character/magicstone?transportID={transport_id}&languageCode=en",
        transport_id = transport_id,
    );

    let response = client.get(request_url).send().await?.text().await?;
    let response_json: MagicStoneResponse = serde_json::from_str(&response)?;

    let magic_stone_collection = database.collection("Magic Stone");

    let record = magic_stone_collection.insert_one(response_json.data, None).await?;
    let filter = doc! { "transport_id": bson::to_bson(transport_id)? };
    let update = doc! { "$set": { "magic_stone_id": record.inserted_id.as_object_id() } };

    nft_collection.update_one(filter, update, None).await?;

    Ok(())
}