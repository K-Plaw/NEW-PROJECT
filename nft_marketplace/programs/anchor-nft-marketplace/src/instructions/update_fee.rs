use anchor_lang::prelude::*;
use anchor_spl::token_interface::TokenInterface;

use crate::{state::Marketplace, error::MarketplaceError, event::FeeUpdated};

/// Accounts required for updating the marketplace fee.
/// This instruction allows the marketplace admin to modify
/// the fee percentage charged on NFT sales.
#[derive(Accounts)]
pub struct UpdateFee<'info> {
    /// The marketplace admin account that has authority to update fees.
    /// Must be the signer of this transaction and match the marketplace's authority.
    #[account(mut)]
    pub admin: Signer<'info>,

    /// The marketplace account whose fee will be updated.
    #[account(
        seeds = [b"marketplace", marketplace.name.as_str().as_bytes()],
        bump = marketplace.marketplace_bump,
    )]
    pub marketplace: Account<'info, Marketplace>,

    /// The Solana System Program
    pub system_program: Program<'info, System>,
    /// The SPL Token Program Interface
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> UpdateFee<'info> {
    /// Updates the marketplace fee to a new value.
    ///
    /// # Arguments
    /// * `updated_fee` - The new fee in basis points (bps) to be charged by the marketplace
    ///                   (e.g., 250 = 2.5%)
    ///
    /// This function updates the fee_bps field in the marketplace account.
    /// The new fee will apply to all subsequent NFT sales in the marketplace.
    pub fn update_fee(&mut self, updated_fee: u16) -> Result<()> {
        // Verify that the admin is the marketplace authority
        require_keys_eq!(self.admin.key(), self.marketplace.authority);

        // Validate that the fee is not too high (max 100%)
        require!(updated_fee <= 10_000, MarketplaceError::FeeTooHigh);

        // Store the old fee for the event
        let old_fee = self.marketplace.fee_bps;

        // Update the fee
        self.marketplace.fee_bps = updated_fee;

        // Emit the fee update event
        emit!(FeeUpdated {
            marketplace: self.marketplace.key(),
            admin: self.admin.key(),
            old_fee,
            new_fee: updated_fee,
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }
}