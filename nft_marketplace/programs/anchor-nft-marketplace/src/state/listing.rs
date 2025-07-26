use anchor_lang::prelude::*;

#[account]
pub struct Listing {
    pub maker: Pubkey,
    pub nft_mint: Pubkey,
    pub price: u64,
    pub metadata: Pubkey,
    pub bump: u8,
}

impl Space for Listing {
    const INIT_SPACE: usize = 8     // discriminator
        + 32                        // maker 
        + 32                        // nft_mint 
        + 8                         // price
        + 32                        // metadata
        + 1;                        // listing bump
}
