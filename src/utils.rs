use mongodb::bson::oid::ObjectId;

#[allow(unused)]
pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub fn object_id() -> ObjectId {
    ObjectId::new()
}
