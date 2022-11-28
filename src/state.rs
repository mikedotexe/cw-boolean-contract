use cosmwasm_schema::cw_serde;
use cw_storage_plus::Item;

#[cw_serde]
pub struct Config {
    pub is_true: bool,
}

// We might as well shorten it to "c" instead of "config"
pub const CONFIG: Item<Config> = Item::new("c");
