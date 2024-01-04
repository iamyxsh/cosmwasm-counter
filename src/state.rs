use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::Item;

pub const OWNER: Item<Addr> = Item::new("owner");
pub const COUNTER: Item<u64> = Item::new("counter");
pub const MINIMAL_TX: Item<Coin> = Item::new("minimal_tx");
