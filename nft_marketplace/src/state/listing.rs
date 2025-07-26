use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use sha2::{Sha256, Digest};

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, Copy)]
pub struct Listing {
    pub maker: Pubkey,
    pub nft_mint: Pubkey,
    pub price: u64,
    pub metadata: Pubkey,
    pub bump: u8,
    pub token_program: Pubkey,
}

impl Listing {
    /// Create a new state.
    pub fn new(
        maker: Pubkey,
        nft_mint: Pubkey,
        price: u64,
        metadata: Pubkey,
        bump: u8,
        token_program: Pubkey,
    ) -> Self {
        Self {
            maker,
            nft_mint,
            price,
            metadata,
            bump,
            token_program,
        }
    }

    /// Total size required for the state account.
    pub const SIZE: usize = 8   // discriminator
        + 32                    // maker
        + 32                    // nft_mint
        + 8                     // price
        + 32                    // metadata
        + 1                     // bump
        + 32;                   // token_program

    /// Returns the Anchor-style 8-byte discriminator for the Listing account.
    pub fn get_discriminator() -> [u8; 8] {
        let mut hasher = Sha256::new();
        hasher.update(b"account:Listing");
        let hash = hasher.finalize();
        let mut discriminator = [0u8; 8];
        discriminator.copy_from_slice(&hash[..8]);
        discriminator
    }
}