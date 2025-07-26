use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, 
    metadata::{MasterEditionAccount, Metadata, MetadataAccount}, 
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked}};

use crate::{event::NFTListed, state::{Listing, Marketplace}, error::MarketplaceError};

/// Accounts required for listing an NFT on the marketplace.
/// This instruction creates a new listing and transfers the NFT
/// from the seller's account to a vault for safekeeping.
#[derive(Accounts)]
pub struct ListNFT<'info> {
    /// The account of the NFT owner who is listing the NFT.
    /// Must be the signer of this transaction and will pay for account creation.
    #[account(mut)]
    pub maker: Signer<'info>,

    /// The marketplace account where the NFT will be listed.
    #[account(
        mut,
        seeds = [b"marketplace", marketplace.name.as_str().as_bytes()],
        bump = marketplace.marketplace_bump,
    )]
    pub marketplace: Account<'info, Marketplace>,

    /// The mint account of the NFT being listed
    #[account(mut)]
    pub maker_nft_mint: InterfaceAccount<'info, Mint>,

    /// The seller's Associated Token Account for the NFT.
    /// This is the account from which the NFT will be transferred.
    #[account(
        mut,
        associated_token::mint = maker_nft_mint,
        associated_token::authority = maker,
    )]
    pub maker_nft_ata: InterfaceAccount<'info, TokenAccount>,

    /// The vault account that will hold the NFT while it's listed.
    /// This is a new Associated Token Account owned by the listing PDA.
    #[account(
        init,
        payer = maker,
        associated_token::mint = maker_nft_mint,
        associated_token::authority = listing,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    /// The listing account that will store information about this sale.
    #[account(
        init,
        payer = maker,
        seeds = [marketplace.key().as_ref(), maker_nft_mint.key().as_ref()],
        bump,
        space = Listing::INIT_SPACE,
    )]
    pub listing: Account<'info, Listing>,

    /// Metadata account of the NFT
    #[account(
        mut,
        seeds = [
            b"metadata", 
            metadata_program.key().as_ref(),
            maker_nft_mint.key().as_ref(),
        ],
        seeds::program = metadata_program.key(),
        bump
    )]
    pub metadata: Account<'info, MetadataAccount>,

    /// Master Edition account of the NFT
    #[account(
        mut,
        seeds = [
            b"metadata", 
            metadata_program.key().as_ref(),
            maker_nft_mint.key().as_ref(),
            b"edition"
        ],
        seeds::program = metadata_program.key(),
        bump,
    )]
    pub master_edition: Account<'info, MasterEditionAccount>,
    
    /// The Metaplex Metadata program
    pub metadata_program: Program<'info, Metadata>,
    /// The SPL Associated Token program
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// The Solana System program
    pub system_program: Program<'info, System>,
    /// The SPL Token program
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> ListNFT<'info> {
    /// Creates a new listing for the NFT with the specified price.
    ///
    /// # Arguments
    /// * `price` - The price in lamports at which the NFT is being listed
    /// * `bumps` - The PDA bump seeds for the listing account
    ///
    /// This function initializes the listing account with the seller's information,
    /// NFT details, and the listing price. It also emits an NFTListed event.
    pub fn create_listing(&mut self, price: u64, bumps: &ListNFTBumps) -> Result<()> {
        // Validate that the price is positive
        require!(price > 0, MarketplaceError::PriceMustBePositive);

        self.listing.set_inner(Listing {
            maker: self.maker.key(),
            nft_mint: self.maker_nft_mint.key(),
            price,
            metadata: self.metadata.key(),
            bump: bumps.listing
        });

        // Verify that the listing owner matches the signer
        require_keys_eq!(self.listing.maker, self.maker.key(), MarketplaceError::InvalidListingOwner);

        // Emit an event that the NFT is listed on the marketplace.
        emit!(NFTListed {
            nft_mint: self.maker_nft_mint.key(),
            marketplace: self.marketplace.key(),
            seller: self.maker.key(),
            price: self.listing.price,
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    /// Transfers the NFT from the seller's account to the vault.
    ///
    /// This function performs a TransferChecked CPI to the Token program
    /// to safely transfer the NFT from the seller's Associated Token Account
    /// to the vault account controlled by the listing PDA.
    pub fn deposit_nft(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        // Prepare the context to be used for the TransferChecked CPI invocation
        let cpi_accounts = TransferChecked {
            from: self.maker_nft_ata.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.maker.to_account_info(),
            mint: self.maker_nft_mint.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        // Transfer the NFT from the maker to the vault
        transfer_checked(cpi_ctx, 1, self.maker_nft_mint.decimals)?;

        Ok(())
    }
}