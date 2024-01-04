mod execute;
mod msg;
mod query;
mod resp;
mod state;
mod utils;

use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use query::query_counter;
use state::{COUNTER, MINIMAL_TX, OWNER};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    COUNTER.save(deps.storage, &msg.initial_value)?;
    OWNER.save(deps.storage, &info.sender)?;
    MINIMAL_TX.save(deps.storage, &msg.minimum_tx)?;

    Ok(Response::new())
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCounter {} => query_counter(deps),
    }
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::IncCounter { value } => execute::inc_counter(deps, info, value),
    }
}

#[cfg(test)]
mod tests {
    use crate::msg::ExecuteMsg;

    use super::*;
    use cosmwasm_std::{coin, coins, Addr};
    use cw_multi_test::{App, ContractWrapper, Executor};

    const MINIMAL_TX_AMOUNT: u128 = 1;
    const MINIMAL_TX_DENOM: &str = "ATOM";
    const CONTRACT_NAME: &str = "Counter Contract";

    const OWNER: &str = "owner";

    fn return_app() -> (App, Addr) {
        let mut app = App::new(|router, _, storage| {
            router
                .bank
                .init_balance(
                    storage,
                    &Addr::unchecked(OWNER),
                    coins(MINIMAL_TX_AMOUNT + 1, MINIMAL_TX_DENOM),
                )
                .unwrap()
        });
        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));
        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &InstantiateMsg {
                    initial_value: 0,
                    minimum_tx: coin(1, "Atom"),
                },
                &[],
                CONTRACT_NAME,
                None,
            )
            .unwrap();
        (app, addr)
    }

    #[test]
    fn it_can_query() {
        let (app, addr) = return_app();

        let resp: u64 = app
            .wrap()
            .query_wasm_smart(addr, &QueryMsg::GetCounter {})
            .unwrap();

        assert_eq!(resp, 0)
    }

    #[test]
    fn it_can_inc_counter() {
        let (mut app, addr) = return_app();

        app.execute_contract(
            Addr::unchecked("owner"),
            addr.clone(),
            &ExecuteMsg::IncCounter { value: 10 },
            &[coin(1, "ATOM")],
        )
        .unwrap();

        let resp: u64 = app
            .wrap()
            .query_wasm_smart(addr, &QueryMsg::GetCounter {})
            .unwrap();

        assert_eq!(resp, 10)
    }
}
