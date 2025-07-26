use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    pubkey::Pubkey,
    program_error::ProgramError,
};
use sha2::{Sha256, Digest};


#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct Marketplace {
    pub authority: Pubkey,
    pub fee_bps: u16,
    pub marketplace_bump: u8,
    pub treasury: Pubkey,
    pub treasury_bump: u8,
    pub name: String, 
}

impl Marketplace {
    /// Maximum allowed length for the marketplace name
    pub const MAX_NAME_LENGTH: usize = 32;

    /// Create a new state.
    pub fn new(
        authority: Pubkey,
        fee_bps: u16,
        marketplace_bump: u8,
        treasury: Pubkey,
        treasury_bump: u8,
        name: String, 
    ) -> Result<Self, ProgramError> {
        // Validate name length
        if name.is_empty() || name.len() > Self::MAX_NAME_LENGTH {
            return Err(ProgramError::InvalidAccountData);
        }

        Ok(Self {
            authority,
            fee_bps,
            marketplace_bump,
            treasury,
            treasury_bump,
            name,
        })
    }

    pub fn size(name: &str) -> usize {
        8 +   // discriminator
        32 +  // authority
        2 +   // fee_bps
        1 +   // marketplace_bump
        32 +  // treasury
        1 +   // treasury_bump
        4 + name.len() // String = 4 bytes length prefix + actual UTF-8 bytes
    }

    /// Returns the Anchor-style 8-byte discriminator for the Marketplace account.
    pub fn get_discriminator() -> [u8; 8] {
        let mut hasher = Sha256::new();
        hasher.update(b"account:Marketplace");
        let hash = hasher.finalize();
        let mut discriminator = [0u8; 8];
        discriminator.copy_from_slice(&hash[..8]);
        discriminator
    }
}