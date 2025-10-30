use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, Decimal};
use cw_storage_plus::Item;

#[cw_serde]
pub struct Config {
    pub owner: Addr,
    pub cw20_funding_token_addr: Option<Addr>,
    pub cw721_fixed_price_addr: Option<Addr>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub price_rate: Option<Decimal>, //just because we will sell bonds for cw20, not for native tokens
    pub outstanding_debt: Option<Coin>,
}

pub const CONFIG: Item<Config> = Item::new("config");
