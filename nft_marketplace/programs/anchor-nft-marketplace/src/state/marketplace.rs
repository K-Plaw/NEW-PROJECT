use anchor_lang::prelude::*;

#[account]
pub struct Marketplace {
    pub authority: Pubkey,
    pub fee_bps: u16, //in hundreds, 250 = 2.5%
    pub marketplace_bump: u8,
    pub treasury: Pubkey,
    pub treasury_bump: u8,
    pub name: String, // Set the limit to 32 bytes
}

impl Space for Marketplace {
    const INIT_SPACE: usize = 8         // discriminator
        + 32                            // authority
        + 2                             // fee_bps
        + 1                             // marketplace_bump
        + 32                            // treasury
        + 1                             // treasury_bump
        + (4 + 32);                     // name (maximum length of 32 characters)
}