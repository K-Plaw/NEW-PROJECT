use anchor_lang::prelude::*;
use anchor_spl::token_interface::TokenInterface;

use crate::{state::Marketplace, error::MarketplaceError};

/// Accounts required for initializing a new marketplace.
/// This instruction creates a new marketplace with specified parameters
/// and initializes necessary PDAs for marketplace operations.
#[derive(Accounts)]
#[instruction(name: String)]
pub struct InitializeMarketplace<'info> {
    /// The admin account that will have authority over the marketplace.
    /// This account must be a signer and will pay for the initialization.
    #[account(mut)]
    pub admin: Signer<'info>,

    /// The marketplace account to be created.
    /// This PDA will store the marketplace configuration and parameters.
    #[account(
        init,
        payer = admin,
        seeds = [b"marketplace", name.as_str().as_bytes()],
        bump,
        space = Marketplace::INIT_SPACE
    )]
    pub marketplace: Account<'info, Marketplace>,

    /// The treasury account for the marketplace.
    /// This PDA will hold the marketplace's collected fees.
    #[account(
        seeds = [b"treasury", marketplace.key().as_ref()],
        bump,
    )]
    pub treasury: SystemAccount<'info>,
    
    /// The Solana System Program
    pub system_program: Program<'info, System>,
    /// The SPL Token Program Interface
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> InitializeMarketplace<'info> {
    /// Initializes a new marketplace with the specified parameters.
    ///
    /// # Arguments
    /// * `name` - The name of the marketplace (must be between 1 and 32 characters)
    /// * `fee` - The marketplace fee in basis points (bps)
    /// * `bumps` - The PDA bump seeds for the marketplace and treasury accounts
    ///
    /// # Errors
    /// * `MarketplaceError::NameNotValid` - If the marketplace name is empty or exceeds 32 characters
    /// * `MarketplaceError::FeeTooHigh` - If the fee exceeds 10,000 basis points (100%)
    pub fn init(&mut self, name: String, fee: u16, bumps: &InitializeMarketplaceBumps) -> Result<()> {
        // Validate that the marketplace name length 
        // is a valid length to be used as a seed for PDA
        require!(name.len() > 0 && name.len() < 33, MarketplaceError::NameNotValid);
        
        // Validate that the fee is not too high (max 100%)
        require!(fee <= 10_000, MarketplaceError::FeeTooHigh);
        
        // Initialize the marketplace account with the provided configuration
        self.marketplace.set_inner(Marketplace {
            authority: self.admin.key(),
            fee_bps: fee,
            marketplace_bump: bumps.marketplace,
            treasury: *self.treasury.key,
            treasury_bump: bumps.treasury,
            name,
        });

        Ok(())
    }
}