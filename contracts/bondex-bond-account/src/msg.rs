use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Coin, Uint128, Decimal};
use cw_utils::Expiration;

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: Option<String>,
    pub owner_addr: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Terminate {},
    UpdateAdmin { admin: Option<String> },
    DoSomething {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
