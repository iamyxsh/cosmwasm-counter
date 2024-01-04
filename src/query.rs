use cosmwasm_std::{to_json_binary, Binary, Deps, StdResult};

use crate::state::COUNTER;

pub fn query_counter(deps: Deps) -> StdResult<Binary> {
    let counter = COUNTER.load(deps.storage)?;
    to_json_binary(&counter)
}
