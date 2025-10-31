// use cosmwasm_schema::{cw_serde, QueryResponses};
// use cosmwasm_std::{Addr, Uint128};
// use crate::api_721_fixed_price::DefaultOptionalNftExtension;
//
// #[cw_serde]
// #[derive(QueryResponses)]
// pub enum QueryMsg {
//     #[returns(ConfigResponse)]
//     GetConfig {},
// }
//
// // #[cw_serde]
// // pub struct ConfigResponse {
// //     pub owner: Addr,
// //     pub cw20_address: Addr,
// //     pub cw721_address: Option<Addr>,
// //     pub max_tokens: u32,
// //     pub unit_price: Uint128,
// //     pub name: String,
// //     pub symbol: String,
// //     pub token_uri: String,
// //     pub extension: DefaultOptionalNftExtension,
// //     pub unused_token_id: u32,
// // }
//
// #[cw_serde]
// pub struct ConfigResponse {
//     pub num_tokens: u64,
//     // pub minter_ownership: Ownership<Addr>,
//     // pub creator_ownership: Ownership<Addr>,
//     pub withdraw_address: Option<String>,
//     pub collection_info: CollectionInfo,
//     pub collection_extension: TCollectionExtension,
//     pub contract_info: ContractInfoResponse,
// }
//
// #[cw_serde]
// #[derive(QueryResponses)]
// pub enum Cw721QueryMsg<
//     // Return type of NFT metadata defined in NftInfo and AllNftInfo.
//     TNftExtension,
//     // Return type of collection extension defined in GetCollectionInfo.
//     TCollectionExtension,
//     // Custom query msg for custom contract logic. Default implementation returns an empty binary.
//     TExtensionQueryMsg,
// > {
//     /// Return the owner of the given token, error if token does not exist
//     #[returns(OwnerOfResponse)]
//     OwnerOf {
//         token_id: String,
//         /// unset or false will filter out expired approvals, you must set to true to see them
//         include_expired: Option<bool>,
//     },
//     /// Return operator that can access all of the owner's tokens.
//     #[returns(ApprovalResponse)]
//     Approval {
//         token_id: String,
//         spender: String,
//         include_expired: Option<bool>,
//     },
//     /// Return approvals that a token has
//     #[returns(ApprovalsResponse)]
//     Approvals {
//         token_id: String,
//         include_expired: Option<bool>,
//     },
//     /// Return approval of a given operator for all tokens of an owner, error if not set
//     #[returns(OperatorResponse)]
//     Operator {
//         owner: String,
//         operator: String,
//         include_expired: Option<bool>,
//     },
//     /// List all operators that can access all of the owner's tokens
//     #[returns(OperatorsResponse)]
//     AllOperators {
//         owner: String,
//         /// unset or false will filter out expired items, you must set to true to see them
//         include_expired: Option<bool>,
//         start_after: Option<String>,
//         limit: Option<u32>,
//     },
//     /// Total number of tokens issued
//     #[returns(NumTokensResponse)]
//     NumTokens {},
//
//     #[deprecated(
//         since = "0.19.0",
//         note = "Please use GetCollectionInfoAndExtension instead"
//     )]
//     #[returns(CollectionInfoAndExtensionResponse<TCollectionExtension>)]
//     /// Deprecated: use GetCollectionInfoAndExtension instead! Will be removed in next release!
//     ContractInfo {},
//
//     /// Returns AllCollectionInfoResponse
//     #[returns(ConfigResponse<TCollectionExtension>)]
//     GetConfig {},
//
//     /// Returns CollectionInfoAndExtensionResponse
//     #[returns(CollectionInfoAndExtensionResponse<TCollectionExtension>)]
//     GetCollectionInfoAndExtension {},
//
//     /// returns AllInfoResponse which contains contract, collection and nft details
//     #[returns(AllInfoResponse)]
//     GetAllInfo {},
//
//     /// Returns CollectionExtensionAttributes
//     #[returns(CollectionExtensionAttributes)]
//     GetCollectionExtensionAttributes {},
//
//     #[deprecated(since = "0.19.0", note = "Please use GetMinterOwnership instead")]
//     #[returns(Ownership<Addr>)]
//     /// Deprecated: use GetMinterOwnership instead! Will be removed in next release!
//     Ownership {},
//
//     /// Return the minter
//     #[deprecated(since = "0.19.0", note = "Please use GetMinterOwnership instead")]
//     #[returns(MinterResponse)]
//     /// Deprecated: use GetMinterOwnership instead! Will be removed in next release!
//     Minter {},
//
//     // #[returns(Ownership<Addr>)]
//     // GetMinterOwnership {},
//     //
//     // #[returns(Ownership<Addr>)]
//     // GetCreatorOwnership {},
//
//     /// With MetaData Extension.
//     /// Returns metadata about one particular token, based on *ERC721 Metadata JSON Schema*
//     /// but directly from the contract
//     #[returns(NftInfoResponse<TNftExtension>)]
//     NftInfo { token_id: String },
//
//     #[returns(Option<NftInfoResponse<TNftExtension>>)]
//     GetNftByExtension {
//         extension: TNftExtension,
//         start_after: Option<String>,
//         limit: Option<u32>,
//     },
//
//     /// With MetaData Extension.
//     /// Returns the result of both NftInfo and OwnerOf as one query as an optimization
//     /// for clients
//     #[returns(AllNftInfoResponse<TNftExtension>)]
//     AllNftInfo {
//         token_id: String,
//         /// unset or false will filter out expired approvals, you must set to true to see them
//         include_expired: Option<bool>,
//     },
//
//     /// With Enumerable extension.
//     /// Returns all tokens owned by the given address, [] if unset.
//     #[returns(TokensResponse)]
//     Tokens {
//         owner: String,
//         start_after: Option<String>,
//         limit: Option<u32>,
//     },
//     /// With Enumerable extension.
//     /// Requires pagination. Lists all token_ids controlled by the contract.
//     #[returns(TokensResponse)]
//     AllTokens {
//         start_after: Option<String>,
//         limit: Option<u32>,
//     },
//
//     /// Custom msg query. Default implementation returns an empty binary.
//     #[returns(())]
//     Extension { msg: TExtensionQueryMsg },
//
//     #[returns(())]
//     GetCollectionExtension { msg: TCollectionExtension },
//
//     #[returns(Option<String>)]
//     GetWithdrawAddress {},
// }
