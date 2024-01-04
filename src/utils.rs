use cosmwasm_std::{Coin, MessageInfo, Uint128};

pub fn calculate_total_funds(info: MessageInfo, min_tx: Coin) -> Uint128 {
    let mut total_atom_funds: Uint128 = Uint128::new(0);
    for coin in info.funds {
        if coin.denom == min_tx.denom {
            total_atom_funds += coin.amount;
        }
    }
    total_atom_funds
}
