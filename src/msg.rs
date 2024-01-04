use serde::{Deserialize, Serialize};

use cosmwasm_std::Coin;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstantiateMsg {
    pub initial_value: u64,
    pub minimum_tx: Coin,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum QueryMsg {
    GetCounter {},
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ExecuteMsg {
    IncCounter { value: u64 },
}
