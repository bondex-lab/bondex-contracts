use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.

    #[error("Cw721FixedPriceAlreadyLinked")]
    Cw721FixedPriceAlreadyLinked {},

    #[error("InvalidTokenReplyId")]
    InvalidTokenReplyId {},

    #[error("Cw20AddressNotSet")]
    Cw20AddressNotSet {},

    #[error("Cw721FixedPriceAddressNotSet")]
    Cw721FixedPriceAddressNotSet {},

    #[error("NothingToWithdraw")]
    NothingToWithdraw {},

    #[error("NoFundsAvailable")]
    NoFundsAvailable {},

    #[error("Cw721FixedPriceNotSet")]
    Cw721FixedPriceNotSet {},

    #[error("InvalidResponse")]
    InvalidResponse {},

    #[error("Cw721BaseAddressNotSet")]
    Cw721BaseAddressNotSet {},

    #[error("NoPaymentDenomSet")]
    NoPaymentDenomSet {},

    #[error("NoActiveDebt")]
    NoActiveDebt {},

    #[error("NoInvestorsFound")]
    NoInvestorsFound {},

    #[error("MissingPriceRate")]
    MissingPriceRate {},

    #[error("NoOutstandingDebt")]
    NoOutstandingDebt {},

    #[error("NotEnoughFundsToPayout")]
    NotEnoughFundsToPayout {},
}
