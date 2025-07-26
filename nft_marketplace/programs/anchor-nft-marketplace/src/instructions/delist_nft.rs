use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, 
    metadata::{MasterEditionAccount, Metadata, MetadataAccount}, 
    token_interface::{transfer_checked, close_account, Mint, TokenAccount, TokenInterface, TransferChecked, CloseAccount}
};

use crate::{event::NFTDelisted, state::{Listing, Marketplace}, error::MarketplaceError};

/// Accounts required for the `delist_nft` instruction.
/// This instruction allows an NFT seller to remove their NFT from the marketplace,
/// retrieving it from the vault and closing associated listing accounts.
#[derive(Accounts)]
pub struct DelistNFT<'info> {
    /// The account of the NFT seller who originally listed the NFT.
    /// Must be the signer of this transaction.
    #[account(mut)]
    pub maker: Signer<'info>,

    /// The marketplace account this NFT was listed on.
    #[account(
        seeds = [b"marketplace", marketplace.name.as_str().as_bytes()],
        bump = marketplace.marketplace_bump,
    )]
    pub marketplace: Account<'info, Marketplace>,
    
    /// The mint account of the NFT being delisted
    pub maker_nft_mint: InterfaceAccount<'info, Mint>,

    /// The maker's Associated Token Account for the NFT.
    /// This is where the NFT will be returned to after delisting.
    #[account(
        mut,
        associated_token::mint = maker_nft_mint,
        associated_token::authority = maker,
    )]
    pub maker_nft_ata: InterfaceAccount<'info, TokenAccount>,

    /// The vault account holding the listed NFT.
    /// This account will be closed after the NFT is returned to the maker.
    #[account(
        mut,
        associated_token::mint = maker_nft_mint,
        associated_token::authority = listing,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    /// The listing account containing the sale information.
    /// This account will be closed and rent will be returned to the maker.
    #[account(
        mut,
        close = maker,
        seeds = [marketplace.key().as_ref(), maker_nft_mint.key().as_ref()],
        bump,
        has_one = maker
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

impl<'info> DelistNFT<'info> {
    /// Withdraws the NFT from the vault back to the maker's Associated Token Account.
    /// This function performs a TransferChecked CPI to the Token program to safely
    /// transfer the NFT and emits an NFTDelisted event.
    pub fn withdraw_nft(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        // Prepare the context to be used for TransferChecked CPI invocation
        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            to: self.maker_nft_ata.to_account_info(),
            authority: self.listing.to_account_info(),
            mint: self.maker_nft_mint.to_account_info(),
        };

        // Construct the signer seeds of the listing account
        let seeds = &[
            self.marketplace.to_account_info().key.as_ref(),
            self.maker_nft_mint.to_account_info().key.as_ref(),
            &[self.listing.bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        // Transfer the NFT from the vault back to the maker
        transfer_checked(cpi_ctx, 1, self.maker_nft_mint.decimals)?;

        // Emit an event that the NFT is delisted.
        // This way we can close the listing account
        // and still track the action that happened.
        emit!(NFTDelisted {
            nft_mint: self.maker_nft_mint.key(),
            marketplace: self.marketplace.key(),
            seller: self.maker.key(),
            price: self.listing.price,
            timestamp: Clock::get()?.unix_timestamp,
        });

        // Listing account automatically closes using the "close" constraint
        // before this instruction is finalized.
        Ok(())
    }

    /// Closes the vault token account and returns the rent to the maker.
    /// This function performs a CloseAccount CPI to the Token program.
    pub fn close_vault(&mut self) -> Result<()> {
        // We cannot close the vault token account automatically
        // by using "close" constraint, so we need to close it manually
        // by using the CloseAccount CPI of Token Program

        let cpi_program = self.token_program.to_account_info();

        // Prepare the context to be used for CloseAccount CPI invocation
        let cpi_accounts = CloseAccount{
            account: self.vault.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.listing.to_account_info()
        };

        // Construct the signer seeds of the listing account
        let seeds = &[
            self.marketplace.to_account_info().key.as_ref(),
            self.maker_nft_mint.to_account_info().key.as_ref(),
            &[self.listing.bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        // Closes that vault token account and send back the rent to the maker
        close_account(cpi_ctx)?;

        Ok(())
    }
}