use schemars::JsonSchema;
use secret_toolkit::storage::Item;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;

pub static CONFIG_KEY: &[u8] = b"config";
pub static CONFIG: Item<State> = Item::new(CONFIG_KEY);

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct State {
    pub count: i32,
    pub owner: Addr,
}

