#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, GrpcQuery, MessageInfo, QueryRequest, Response, StdResult, to_json_binary, Decimal, Uint128, WasmMsg, SubMsg, Coin, Reply, Addr, BankMsg};
use cw20::BalanceResponse;
use cw2::set_contract_version;
use cw_utils::parse_instantiate_response_data;
use crate::api_721_fixed_price;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, ConfigResponse, InstantiateBondMsg};
use cw721_base::msg as cw721_base_msg;
use cw721::msg as cw721_msg;
use schemars::_serde_json::json;
use crate::error::ContractError;
use crate::state::{Config, CONFIG};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:bondex-bond-account";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const INSTANTIATE_BOND_SERIES_REPLY_ID: u64 = 1;

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
        ExecuteMsg::WithdrawFunds {} => execute_withdraw_funds(deps, env, info),
        ExecuteMsg::PayoutBonds {} => execute_payout_bonds(deps, env, info),
        ExecuteMsg::IssueBondSeries {
            name,
            cw20_funding_token_addr,
            price_rate,
            number_of_bonds,
            price_per_bond,
            bond_nft_code_id,
            bond_nft_fixed_price_code_id,
            symbol,
            token_uri,
            debt_payment_denom,
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
            bond_nft_fixed_price_code_id,
            symbol,
            token_uri,
            debt_payment_denom
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
    bond_nft_fixed_price_code_id: u64,
    symbol: String,
    token_uri: String,
    debt_payment_denom: String,
) -> Result<Response, ContractError> {
    //validate: only an owner can call this
    let state = CONFIG.load(deps.storage)?;
    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {});
    }

    // instantiate cw721fixed_price
    let instantiate_msg = InstantiateBondMsg {
        // owner: info.sender.clone(),
        owner: state.owner.to_string(),
        max_tokens: number_of_bonds,
        unit_price: price_per_bond,
        name: name.clone(),
        symbol: symbol.clone(),
        token_code_id: bond_nft_code_id,
        cw20_address: deps.api.addr_validate(&cw20_funding_token_addr)?,
        token_uri: token_uri.clone(),
    };

    let instantiate_msg_bin = to_json_binary(&instantiate_msg)?;
    let instantiate_submsg = SubMsg::reply_on_success(
        WasmMsg::Instantiate {
            admin: Some(state.owner.to_string()),
            code_id: bond_nft_fixed_price_code_id,
            msg: instantiate_msg_bin,
            funds: vec![],
            label: format!("Bond series: {}", name),
        },
        INSTANTIATE_BOND_SERIES_REPLY_ID,
    );

    // assemble and store config
    let cfg = Config {
        owner: state.owner,
        cw20_funding_token_addr: Some(deps.api.addr_validate(&cw20_funding_token_addr)?),
        cw721_fixed_price_addr: None,
        title: Some(name),
        description: None,
        price_rate: Some(price_rate),
        outstanding_debt: Some(Coin{
            denom: debt_payment_denom,
            amount: Uint128::from(number_of_bonds) * price_per_bond
        }),
    };
    CONFIG.save(deps.storage, &cfg)?;

    Ok(Response::new()
        .add_submessage(instantiate_submsg)
        .add_attribute("action", "issue_bond_series")
        .add_attribute("creator", info.sender)
        .add_attribute("cw20_funding_token", cw20_funding_token_addr)
        .add_attribute("price_per_bond", price_per_bond)
        .add_attribute("number_of_bonds", number_of_bonds.to_string()))
}

// Reply callback triggered from cw721 filex price contract instantiation
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    let mut config: Config = CONFIG.load(deps.storage)?;

    if config.cw721_fixed_price_addr.is_some() {
        return Err(ContractError::Cw721FixedPriceAlreadyLinked {});
    }

    if msg.id != INSTANTIATE_BOND_SERIES_REPLY_ID {
        return Err(ContractError::InvalidTokenReplyId {});
    }
    let result = msg.result.into_result().unwrap();
    let data = result.msg_responses.first().unwrap();
    let reply = parse_instantiate_response_data(data.value.as_slice()).unwrap();
    config.cw721_fixed_price_addr = Addr::unchecked(reply.contract_address).into();
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new())
}

fn execute_payout_bonds(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let cfg = CONFIG.load(deps.storage)?;

    let debt = cfg
        .outstanding_debt
        .ok_or(ContractError::NoOutstandingDebt {})?;

    let native_balance: Coin = deps
        .querier
        .query_balance(
            env.contract.address.clone(),
            debt.denom
        )?;

    if native_balance.amount.is_zero() {
        return Err(ContractError::NoFundsAvailable {});
    }

    let fixed_price_addr = cfg
        .cw721_fixed_price_addr
        .ok_or(ContractError::Cw721FixedPriceNotSet {})?;

    let config_721_fixed_price_resp: api_721_fixed_price::ConfigResponse = deps.querier.query_wasm_smart(
        fixed_price_addr.clone(),
        &api_721_fixed_price::QueryMsg::GetConfig {},
    )?;

    let cw721_base_addr = config_721_fixed_price_resp
        .cw721_address
        .ok_or(ContractError::Cw721BaseAddressNotSet {})?;

    let tokens_resp: cw721_msg::TokensResponse = deps.querier.query_wasm_smart(
        cw721_base_addr.clone(),
        &cw721_base_msg::QueryMsg::AllTokens {
            start_after: None,
            limit: None
        },
    )?;

    let mut owners: Vec<Addr> = Vec::new();
    let number_of_investors : Uint128 = Uint128::new(tokens_resp.tokens.len() as u128);
    for token_id in tokens_resp.tokens {
        let bond_resp: cw721_msg::OwnerOfResponse = deps.querier.query_wasm_smart(
            cw721_base_addr.clone(),
            &cw721_base_msg::QueryMsg::OwnerOf {
                token_id,
                include_expired: None,
            },
        )?;
        let investor_addr = deps.api.addr_validate(&bond_resp.owner)?;
        owners.push(investor_addr);
    }

    if owners.is_empty() {
        return Err(ContractError::NoInvestorsFound {});
    }

    let rate = cfg
        .price_rate
        .ok_or(ContractError::MissingPriceRate {})?;

    let payout_avg = {
        let as_decimal = Decimal::from_ratio(debt.amount, Uint128::new(1));
        let payout_dec = as_decimal * rate;
        payout_dec.to_uint_floor()
    };

    if payout_avg <= Uint128::new(0) {
        return Err(ContractError::NotEnoughFundsToPayout {})
    }

    let total_payout = payout_avg * number_of_investors;

    let mut msgs: Vec<BankMsg> = Vec::new();
    for owner in owners {
        msgs.push(BankMsg::Send {
            to_address: owner.to_string(),
            amount: vec![Coin {
                denom: native_balance.denom.clone(),
                amount: payout_avg,
            }],
        });
    }

    Ok(Response::new()
        .add_messages(msgs)
        .add_attribute("action", "payout_bonds")
        .add_attribute("recipients", number_of_investors.to_string())
        .add_attribute("each_payout", payout_avg.to_string()))
}

fn execute_withdraw_funds(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    // todo withdraw native tokens over debt

    let cfg = CONFIG.load(deps.storage)?;

    // only owner can call this
    if info.sender != cfg.owner {
        return Err(ContractError::Unauthorized {});
    }

    let cw20_addr = cfg
        .cw20_funding_token_addr
        .ok_or(ContractError::Cw20AddressNotSet {})?;

    let cw721_fixed_price_addr = cfg
        .cw721_fixed_price_addr
        .ok_or(ContractError::Cw721FixedPriceAddressNotSet {})?;

    let balance: BalanceResponse = deps.querier.query_wasm_smart(
        cw20_addr.clone(),
        &cw20::Cw20QueryMsg::Balance {
            address: cw721_fixed_price_addr.to_string(),
        },
    )?;

    // let transfer_msg = WasmMsg::Execute { //todo: fix it! not from current contract, but from fixed price!
    //     contract_addr: cw20_addr.to_string(),
    //     msg: to_json_binary(&cw20::Cw20ExecuteMsg::Transfer {
    //         recipient: cfg.owner.to_string(),
    //         amount: balance.balance,
    //     })?,
    //     funds: vec![],
    // };

    // withdraw all cw20 tokens from cw721_fixed_price to owner
    let transfer_msg = WasmMsg::Execute {
        contract_addr: cw20_addr.to_string(),
        msg: to_json_binary(&api_721_fixed_price::ExecuteMsg::TransferFundsToOwner {})?,
        funds: vec![],
    };

    // withdraw native tokens
    let mut result = Response::new();
    let native_balance: Coin = deps
        .querier
        .query_balance(
            env.contract.address.clone(),
            cfg.outstanding_debt.clone().unwrap().denom
        )?;

    if !native_balance.amount.is_zero() {
        let debt = cfg.outstanding_debt;

        if debt.is_none() {
            let msg = BankMsg::Send {
                to_address: cfg.owner.to_string(),
                amount: vec![native_balance.clone()],
            };

            result = result
                .add_message(msg)
                .add_attribute("action", "withdraw_all_native")
                .add_attribute("recipient", cfg.owner.to_string())
                .add_attribute("native_amount", native_balance.amount.to_string())
                .add_attribute("native_denom", native_balance.denom.to_string());
        } else {
            let debt_token = debt.unwrap();
            let payment_amount = native_balance.amount - debt_token.amount;
            let msg = BankMsg::Send {
                to_address: cfg.owner.to_string(),
                amount: vec![Coin::new(payment_amount, debt_token.denom)],
            };

            result = result
                .add_message(msg)
                .add_attribute("action", "withdraw_available_after_debt_native")
                .add_attribute("recipient", cfg.owner.to_string())
                .add_attribute("native_amount", native_balance.amount.to_string())
                .add_attribute("native_denom", native_balance.denom.to_string());
        }
    }


    Ok(result
        .add_message(transfer_msg)
        .add_attribute("action", "withdraw_cw20_funds")
        .add_attribute("recipient", cfg.owner)
        .add_attribute("amount_cw20", balance.balance))
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
