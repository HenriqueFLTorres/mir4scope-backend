use std::collections::HashMap;

use super::inventory::InventoryItem;

pub async fn get_nft_tickets(
    inventory: Vec<InventoryItem>,
) -> anyhow::Result<HashMap<String, i32>> {
    let mut tickets: HashMap<String, i32> = HashMap::new();

    inventory.iter().for_each(|x| match x.item_name.as_str() {
        "Wayfarer Travel Pass"
        | "Secret Peak Ticket"
        | "Magic Square Ticket"
        | "Raid Ticket"
        | "Boss Raid Ticket"
        | "Hell Raid Ticket" => {
            tickets.insert(x.item_name.clone(), x.stack);
        }
        _ => {}
    });

    Ok(tickets)
}
