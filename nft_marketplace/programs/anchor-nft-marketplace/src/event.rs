use anchor_lang::prelude::*;

#[event]
pub struct NFTListed {
    pub nft_mint: Pubkey,
    pub marketplace: Pubkey,
    pub seller: Pubkey,
    pub price: u64,
    pub timestamp: i64,
}

#[event]
pub struct NFTPurchased {
    pub nft_mint: Pubkey,
    pub marketplace: Pubkey,
    pub seller: Pubkey,
    pub buyer: Pubkey,
    pub price: u64,
    pub timestamp: i64,
}

#[event]
pub struct NFTDelisted {
    pub nft_mint: Pubkey,
    pub marketplace: Pubkey,
    pub seller: Pubkey,
    pub price: u64,
    pub timestamp: i64,
}

#[event]
pub struct FeeUpdated {
    pub marketplace: Pubkey,
    pub admin: Pubkey,
    pub old_fee: u16,
    pub new_fee: u16,
    pub timestamp: i64,
}
