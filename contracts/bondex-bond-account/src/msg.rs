use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Coin, Uint128, Decimal, Addr};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner_addr: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Terminate {},
    UpdateAdmin { admin: Option<String> },
    WithdrawFunds {},
    PayoutBonds {},
    IssueBondSeries {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    GetConfig {},
}

#[cw_serde]
pub struct ConfigResponse {
    pub owner_addr: Addr,
    pub cw20_funding_token_addr: Option<Addr>,
    pub cw721_fixed_price_addr: Option<Addr>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub price_rate: Option<Decimal>, //just because we will sell bonds for cw20, not for native tokens
    pub outstanding_debt: Option<Coin>,
}
