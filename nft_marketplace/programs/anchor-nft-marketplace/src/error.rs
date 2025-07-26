use anchor_lang::error_code;

#[error_code]
pub enum MarketplaceError {
    #[msg("The name shouldn't be empty or exceed 32 characters.")]
    NameNotValid,
    #[msg("The fee cannot exceed 10,000 basis points (100%).")]
    FeeTooHigh,
    #[msg("The listing price must be greater than zero.")]
    PriceMustBePositive,
    #[msg("The listing owner does not match the signer.")]
    InvalidListingOwner,
}