use anchor_lang::prelude::*;

pub mod error;
pub mod state;
pub mod event;
pub mod instructions;

pub use instructions::*;

declare_id!("FvdEiEPJUEMUZ7HCkK2gPfYGFXCbUB68mTJufdC9BjC5");

#[program]
pub mod anchor_nft_marketplace {
    use super::*;

    pub fn initialize_marketplace(ctx: Context<InitializeMarketplace>, name: String, fee: u16) -> Result<()> {
        ctx.accounts.init(name, fee, &ctx.bumps)?;

        Ok(())
    }

    pub fn list_nft(ctx: Context<ListNFT>, price: u64) -> Result<()> {
        ctx.accounts.create_listing(price, &ctx.bumps)?;
        ctx.accounts.deposit_nft()?;

        Ok(())
    }  

    pub fn purchase_nft(ctx: Context<Purchase>) -> Result<()> {
        ctx.accounts.send_fee_to_maker()?;
        ctx.accounts.send_fee_to_treasury()?;
        ctx.accounts.transfer_nft()?;
        ctx.accounts.close_vault()?;

        Ok(())
    }

    pub fn delist_nft(ctx: Context<DelistNFT>) -> Result<()> {
        ctx.accounts.withdraw_nft()?;
        ctx.accounts.close_vault()?;

        Ok(())
    }

    pub fn update_fee(ctx: Context<UpdateFee>, updated_fee: u16) -> Result<()> {
        ctx.accounts.update_fee(updated_fee)?;

        Ok(())
    }



}
