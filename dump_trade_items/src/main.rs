use serde_json::{json, Value};
use std::{collections::HashMap, fs};

fn main() -> serde_json::Result<()> {
    let data = fs::read_to_string("/ITEM.json").unwrap();
    let serde_value: Value = serde_json::from_str(&data).expect("ITEM.json file was not found");

    let mut new_data = HashMap::new();
    if let Value::Object(obj) = &serde_value[0]["Rows"] {
        for (item_id, item_object) in obj {
            new_data.insert(item_id, item_object["TradeType"].clone());
        }
    }

    let new_json = json!(new_data);
    fs::write(
        "./list.json",
        serde_json::to_string_pretty(&new_json)?,
    )
    .unwrap();

    Ok(())
}
