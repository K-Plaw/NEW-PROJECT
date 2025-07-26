use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Debug, Clone, Copy, Error, PartialEq, Eq)]
pub enum MarketplaceError {
    #[error("The given name is longer than 32 characters.")]
    NameTooLong,

    #[error("Math operation overflowed.")]
    MathOverflow,

    #[error("Invalid fee value. Must be between 1 and 10000 bps.")]
    InvalidFee,

    #[error("Marketplace is already initialized.")]
    AlreadyInitialized,

    #[error("Listing account related to NFT already exists.")]
    ListingAlreadyExists,

    #[error("Invalid marketplace account.")]
    InvalidMarketplaceAccount,

    #[error("The price that has been passed is invalid.")]
    InvalidPrice,

    #[error("Invalid listing account.")]
    InvalidListingAccount,

    #[error("Invalid vault owner.")]
    InvalidVaultOwner,

    #[error("You are not the owner of this listing account.")]
    InvalidListingOwner,

    #[error("Invalid token amount.")]
    InvalidTokenAmount,

    #[error("Invalid token mint.")]
    InvalidTokenMint,

    #[error("Invalid token account.")]
    InvalidTokenAccount,

    #[error("Invalid token state.")]
    InvalidTokenState,

   
}

impl From<MarketplaceError> for ProgramError {
    fn from(error: MarketplaceError) -> Self {
        ProgramError::Custom(error as u32)
    }
}