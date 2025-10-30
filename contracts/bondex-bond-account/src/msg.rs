use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Coin, Uint128, Decimal, Addr};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner_addr: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    WithdrawFunds {},
    PayoutBonds {},
    IssueBondSeries {
        /// Name of the NFT contract. todo: Collection or exact NFT?
        name: String,
        cw20_funding_token_addr: String,
        price_rate: Decimal, //Just because we will sell bonds for cw20, not for native tokens. We need to convert one to the other with rate
        number_of_bonds: u32,
        price_per_bond: Uint128, //price for cw20 tokens
        bond_nft_code_id: u64, // cw721_base_code_id todo: normal way is to get it from the factory contract managed by protocol team
        bond_nft_fixed_price_code_id: u64, // cw721_fixed_price_code_id todo: normal way is to get it from the factory contract managed by protocol team
        symbol: String,
        token_uri: String,
    },
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
