use std::cmp::Ordering;

use cosmwasm_std::{DepsMut, MessageInfo, Response, StdError, StdResult};

use crate::{
    state::{COUNTER, MINIMAL_TX, OWNER},
    utils,
};

pub fn inc_counter(deps: DepsMut, info: MessageInfo, value: u64) -> StdResult<Response> {
    let owner = OWNER.load(deps.storage)?;
    if owner != info.sender {
        return Err(StdError::GenericErr {
            msg: "invalid sender".to_string(),
        });
    }
    let min_tx = MINIMAL_TX.load(deps.storage)?;
    let atom_funds = utils::calculate_total_funds(info, min_tx.clone());
    if min_tx.amount.cmp(&atom_funds) == Ordering::Less {
        return Err(StdError::GenericErr {
            msg: "send minimal tx".to_string(),
        });
    }
    COUNTER.save(deps.storage, &value)?;
    Ok(Response::new())
}
