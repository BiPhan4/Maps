use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub count: i32,
    pub owner: Addr,
}

pub struct Entry {
    
}


pub const STATE: Item<State> = Item::new("state");
//given name to our State item is "state" 

pub const ENTRIES: Map<&Addr, u8> = Map::new("entries"); 
//this is no. of entries that address has made into our raffle














