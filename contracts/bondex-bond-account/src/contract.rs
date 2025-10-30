#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, GrpcQuery, MessageInfo, QueryRequest, Response, StdResult, to_json_binary, Decimal, Uint128};
use cw2::set_contract_version;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, ConfigResponse};

use crate::error::ContractError;
use crate::state::{Config, CONFIG};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:bondex-bond-account";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // address normalization
    let owner = deps.api.addr_validate(&msg.owner_addr)?;

    // assemble and store config
    let cfg = Config {
        owner,
        cw20_funding_token_addr: None,
        cw721_fixed_price_addr: None,
        title: None,
        description: None,
        price_rate: None,
        outstanding_debt: None,
    };
    CONFIG.save(deps.storage, &cfg)?;

    // version for migrations
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new().add_attribute("action", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        // ExecuteMsg::DoSomething {
        // } => execute_try_grpc(deps, env, info),
        ExecuteMsg::Terminate {} => unimplemented!(),
        ExecuteMsg::UpdateAdmin {
            admin
        } => unimplemented!(),
        ExecuteMsg::WithdrawFunds {} => execute_withdraw_funds(deps, env, info),
        ExecuteMsg::PayoutBonds {} => execute_payout_bonds(deps, env, info),
        ExecuteMsg::IssueBondSeries {
            name,
            cw20_funding_token_addr,
            price_rate,
            number_of_bonds,
            price_per_bond,
            bond_nft_code_id,
            symbol,
            token_uri
        } => execute_issue_bond_series(
            deps,
            env,
            info,
            name,
            cw20_funding_token_addr,
            price_rate,
            number_of_bonds,
            price_per_bond,
            bond_nft_code_id,
            symbol,
            token_uri
        ),
    }
}

fn execute_issue_bond_series(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    name: String,
    cw20_funding_token_addr: String,
    price_rate: Decimal,
    number_of_bonds: u32,
    price_per_bond: Uint128,
    bond_nft_code_id: u64,
    symbol: String,
    token_uri: String,
) -> Result<Response, ContractError> {
    //todo: validate - only an owner can call this

    // todo: instantiate cw721fixed_price
    // todo: save some params to the contract state

    unimplemented!()
}

fn execute_payout_bonds(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    unimplemented!()
}

fn execute_withdraw_funds(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    unimplemented!()
}

fn execute_try_grpc(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {

    // let q = QueryRequest::Grpc{
    //     0: GrpcQuery {
    //         path: "/cosmos.protocolpool.v1.Query/ContinuousFunds".to_string(),
    //         data: Default::default()
    //     }
    // };
    // // QueryRequest::Custom {}
    // let resp_bin: Binary = deps.querier.query(&q)?;
    // Ok(Response::new()
    //     .set_data(resp_bin.clone())
    //     .add_attribute("raw_response", base64::encode(resp_bin)))
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetConfig {} => to_json_binary(&query_config(deps)?),
    }
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse {
        owner_addr: config.owner,
        cw20_funding_token_addr: config.cw20_funding_token_addr,
        cw721_fixed_price_addr: config.cw721_fixed_price_addr,
        title: config.title,
        description: config.description,
        price_rate: config.price_rate,
        outstanding_debt: config.outstanding_debt,
    })
}

#[cfg(test)]
mod tests {
}
