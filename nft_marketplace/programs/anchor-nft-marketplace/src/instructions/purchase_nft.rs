use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};
use anchor_spl::{
    associated_token::AssociatedToken, 
    token_interface::{
        transfer_checked, 
        close_account, 
        CloseAccount, 
        Mint, 
        TokenAccount, 
        TokenInterface, 
        TransferChecked}
    };

use crate::{
    state::{Listing, Marketplace},
    event::NFTPurchased
};

/// Accounts required for purchasing an NFT from the marketplace.
/// This instruction handles the complete purchase flow including
/// fee payments, NFT transfer, and cleanup of listing accounts.
#[derive(Accounts)]
pub struct Purchase<'info> {
    /// The buyer's account, must be the signer and will pay for the NFT
    /// and any account creation costs
    #[account(mut)]
    pub taker: Signer<'info>,

    /// The seller's account that will receive payment for the NFT
    #[account(mut)]
    pub maker: SystemAccount<'info>,

    /// The mint account of the NFT being purchased
    pub maker_nft_mint: InterfaceAccount<'info, Mint>,

    /// The marketplace account where the NFT is listed
    #[account(
        seeds = [b"marketplace", marketplace.name.as_str().as_bytes()],
        bump = marketplace.marketplace_bump,
    )]
    pub marketplace: Account<'info, Marketplace>,

    /// The buyer's Associated Token Account for the NFT.
    /// Will be initialized if it doesn't exist yet.
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = maker_nft_mint,
        associated_token::authority = taker,
    )]
    pub taker_ata: InterfaceAccount<'info, TokenAccount>,

    /// The vault account currently holding the listed NFT
    #[account(
        mut,
        associated_token::mint = maker_nft_mint,
        associated_token::authority = listing,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    /// The listing account containing the sale information
    /// Will be closed after successful purchase
    #[account(
        mut,
        close = maker,
        seeds = [marketplace.key().as_ref(), maker_nft_mint.key().as_ref()],
        bump = listing.bump,
    )]
    pub listing: Account<'info, Listing>,

    /// The marketplace treasury account that receives listing fees
    /// Seeds = ["treasury", marketplace.key()]
    #[account(
        mut,
        seeds = [b"treasury", marketplace.key().as_ref()],
        bump = marketplace.treasury_bump,
    )]
    pub treasury: SystemAccount<'info>,

    /// The SPL Associated Token program
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// The SPL Token program
    pub token_program: Interface<'info, TokenInterface>,
    /// The Solana System program
    pub system_program: Program<'info, System>,
}

impl<'info> Purchase <'info> {
    /// Transfers the payment (minus marketplace fee) to the seller.
    ///
    /// Calculates the final amount by subtracting the marketplace fee
    /// from the listing price and transfers it to the seller.
    pub fn send_fee_to_maker(&mut self) -> Result <()> {
        let cpi_program = self.system_program.to_account_info();

        // Preparing the context to be used for Transfer CPI invocation
        let cpi_accounts = Transfer {
            from: self.taker.to_account_info(),
            to: self.maker.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        // Calculate the proper amount to transfer minus the marketplace fee
        let price = self.listing.price.clone();
        let fee = self.marketplace.fee_bps.clone();

        let calculated_amount = price.checked_sub(
            price.checked_mul(fee as u64).unwrap().checked_div(10_000).unwrap()
        ).unwrap();

        // Transfer SOL equal to the listing price (minus the marketplace fee)
        // from the customer to the maker
        transfer(cpi_ctx, calculated_amount)?;
        Ok(())
    }

    /// Transfers the marketplace fee to the treasury account.
    ///
    /// Calculates the marketplace fee based on the listing price and
    /// the marketplace's fee_bps, then transfers it to the treasury.
    pub fn send_fee_to_treasury(&mut self) -> Result <()> {
        let cpi_program = self.system_program.to_account_info();

        // Prepare the context to be used for Transfer CPI invocation
        let cpi_accounts = Transfer{
            from: self.taker.to_account_info(),
            to: self.treasury.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        // Get the value of the listing price and the marketplace fee
        let price = self.listing.price.clone();
        let fee = self.marketplace.fee_bps.clone();

        // Calculating the proper fee to be sent to the treasury
        let calculated_fee = price.checked_mul(fee.into()).unwrap().checked_div(10_000).unwrap();

        // Transfer SOL equal to the marketplace fee
        // from the customer to the marketplace treasury
        transfer(cpi_ctx, calculated_fee)?;
        
        Ok(())
    }

    /// Transfers the NFT from the vault to the buyer's account.
    ///
    /// Performs a TransferChecked CPI to move the NFT from the vault
    /// to the buyer's Associated Token Account and emits an NFTPurchased event.
    pub fn transfer_nft(&mut self) -> Result <()> {
        let cpi_program = self.token_program.to_account_info();

        // Prepare the context to be used for TransferChecked CPI Invocation
        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            mint: self.maker_nft_mint.to_account_info(),
            to: self.taker_ata.to_account_info(),
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

        // Transfer the NFT from the vault to the customer
        transfer_checked(cpi_ctx, 1, self.maker_nft_mint.decimals)?;

        // Emit an event that the NFT is purchased so that we won't need to retain
        // the listing account, and have that rent be given back to the seller.
        emit!(NFTPurchased {
            nft_mint: self.maker_nft_mint.key(),
            marketplace: self.marketplace.key(),
            seller: self.maker.key(),
            buyer: self.taker.key(),
            price: self.listing.price,
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    /// Closes the vault token account and returns the rent to the seller.
    ///
    /// Performs a CloseAccount CPI to close the vault token account
    /// and return its rent to the seller after the NFT has been transferred.
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