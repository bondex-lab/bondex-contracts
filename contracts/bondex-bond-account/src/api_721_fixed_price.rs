use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};

#[cw_serde]
pub enum ExecuteMsg {
    // Receive(Cw20ReceiveMsg),
    TransferFundsToOwner {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    GetConfig {},
}

#[cw_serde]
pub struct ConfigResponse {
    pub owner: Addr,
    pub cw20_address: Addr,
    pub cw721_address: Option<Addr>,
    pub max_tokens: u32,
    pub unit_price: Uint128,
    pub name: String,
    pub symbol: String,
    pub token_uri: String,
    pub extension: DefaultOptionalNftExtension,
    pub unused_token_id: u32,
}

pub type DefaultOptionalNftExtension = Option<NftExtension>;

#[cw_serde]
#[derive(Default)]
pub struct NftExtension {
    pub image: Option<String>,
    pub image_data: Option<String>,
    pub external_url: Option<String>,
    pub description: Option<String>,
    pub name: Option<String>,
    pub attributes: Option<Vec<Trait>>,
    pub background_color: Option<String>,
    pub animation_url: Option<String>,
    pub youtube_url: Option<String>,
}

#[cw_serde]
pub struct Trait {
    pub display_type: Option<String>,
    pub trait_type: String,
    pub value: String,
}
