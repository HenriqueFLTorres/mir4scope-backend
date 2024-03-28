use crate::responses::nft::Nft;
use mongodb::{bson::oid::ObjectId, Collection, Database};

#[allow(unused)]
pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub fn object_id() -> ObjectId {
    ObjectId::new()
}

pub struct State {
    pub nft_collection: Collection<Nft>,
    pub client: reqwest::Client,
    pub database: Database,
}

impl State {}
