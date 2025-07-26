use borsh::BorshDeserialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    program_pack::Pack
};

use spl_token::{
    instruction::{transfer_checked as spl_transfer_checked, close_account as spl_close_account},
    state::Account as TokenAccount,
    ID as TOKEN_PROGRAM_ID,
};

use spl_token_2022::instruction::{
    transfer_checked as spl_2022_transfer_checked, 
    close_account as spl_2022_close_account
};

use crate::{
    state::{Listing, Marketplace},
    error::MarketplaceError
}; // borsh-deserializable structs

/// Maximum allowed fee in basis points (100%)
const MAX_FEE_BPS: u16 = 10_000;

/// NFT decimals should always be 0
const NFT_DECIMALS: u8 = 0;

/// Instruction: Purchase NFT on the marketplace
pub fn process_purchase(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    // Validate account count
    if accounts.len() != 10 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    let account_info_iter = &mut accounts.iter();

    // Buyer paying for the NFT
    let taker = next_account_info(account_info_iter)?; // signer

    // Seller receiving payout
    let maker = next_account_info(account_info_iter)?;

    // Mint of the NFT being sold
    let maker_nft_mint = next_account_info(account_info_iter)?;

    // Marketplace account (PDA)
    let marketplace = next_account_info(account_info_iter)?;

    // Buyer's ATA for receiving the NFT
    let taker_ata = next_account_info(account_info_iter)?;

    // Vault holding the NFT (ATA owned by listing PDA)
    let vault = next_account_info(account_info_iter)?;

    // Listing PDA holding sale config + authority over vault
    let listing = next_account_info(account_info_iter)?;

    // Treasury (marketplace fee receiver)
    let treasury = next_account_info(account_info_iter)?;
    
    // Token + System programs
    let token_program = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    // === Critical Account Validations ===
    
    // Validate program IDs
    if system_program.key != &solana_program::system_program::id() {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Validate marketplace is owned by this program
    if marketplace.owner != program_id {
        return Err(ProgramError::IllegalOwner);
    }

    // Validate listing is owned by this program
    if listing.owner != program_id {
        return Err(ProgramError::IllegalOwner);
    }

    // Verify listing account discriminator
    let listing_data_bytes = listing.data.borrow();
    if listing_data_bytes[0..8] != Listing::get_discriminator()[..8] {
        return Err(ProgramError::InvalidAccountData);
    }

    // Validate signer
    if !taker.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Deserialize listing & marketplace data from their accounts
    let listing_data = Listing::try_from_slice(&listing_data_bytes[8..])?;
    let marketplace_data = Marketplace::try_from_slice(&marketplace.data.borrow())?;

    // Validate marketplace fee is within allowed range
    if marketplace_data.fee_bps > MAX_FEE_BPS {
        return Err(MarketplaceError::InvalidFee.into());
    }

    // Validate token program matches the one used during listing
    if *token_program.key != listing_data.token_program {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Validate token accounts are owned by the correct token program
    if vault.owner != token_program.key {
        return Err(ProgramError::IllegalOwner);
    }
    if taker_ata.owner != token_program.key {
        return Err(ProgramError::IllegalOwner);
    }

    // Verify vault token state
    let vault_data = TokenAccount::unpack(&vault.try_borrow_data()?)?;
    if vault_data.amount != 1 {
        return Err(MarketplaceError::InvalidTokenAmount.into());
    }
    if vault_data.mint != *maker_nft_mint.key {
        return Err(MarketplaceError::InvalidTokenMint.into());
    }
    // Verify vault authority
    if vault_data.owner != *listing.key {
        return Err(MarketplaceError::InvalidVaultOwner.into());
    }

    // Verify taker's ATA
    let expected_taker_ata = spl_associated_token_account::get_associated_token_address(
        taker.key,
        maker_nft_mint.key,
    );
    if *taker_ata.key != expected_taker_ata {
        return Err(MarketplaceError::InvalidTokenAccount.into());
    }

    // Verify vault ATA
    let expected_vault = spl_associated_token_account::get_associated_token_address(
        listing.key,
        maker_nft_mint.key,
    );
    if *vault.key != expected_vault {
        return Err(MarketplaceError::InvalidTokenAccount.into());
    }

    // Validate NFT mint matches listing data
    if *maker_nft_mint.key != listing_data.nft_mint {
        return Err(ProgramError::InvalidAccountData);
    }

    // Verify marketplace PDA
    let marketplace_data = Marketplace::try_from_slice(&marketplace.data.borrow())?;
    let (marketplace_pda, _) = Pubkey::find_program_address(
        &[b"marketplace", marketplace_data.name.as_bytes()],
        program_id,
    );
    if marketplace_pda != *marketplace.key {
        return Err(MarketplaceError::InvalidMarketplaceAccount.into());
    }

    // Validate treasury PDA derivation
    let (expected_treasury, _) = Pubkey::find_program_address(
        &[b"treasury", marketplace.key.as_ref()],
        program_id,
    );
    if *treasury.key != expected_treasury {
        return Err(ProgramError::InvalidSeeds);
    }

    // Validate listing PDA derivation
    let (expected_listing, listing_bump) = Pubkey::find_program_address(
        &[marketplace.key.as_ref(), maker_nft_mint.key.as_ref()],
        program_id,
    );
    if *listing.key != expected_listing {
        return Err(ProgramError::InvalidSeeds);
    }
    if listing_data.bump != listing_bump {
        return Err(ProgramError::InvalidSeeds);
    }

    // Validate maker matches listing data
    if *maker.key != listing_data.maker {
        return Err(ProgramError::InvalidAccountData);
    }

    // Validate taker is not the maker (prevent self-purchase)
    if *taker.key == *maker.key {
        return Err(ProgramError::InvalidAccountData);
    }

    // Validate taker has enough lamports for the purchase
    let price = listing_data.price;
    let fee_bps = marketplace_data.fee_bps;

    // Calculate fee (bps-based) and seller payout
    let fee = price
        .checked_mul(fee_bps as u64)
        .ok_or(MarketplaceError::MathOverflow)?
        .checked_div(10_000)
        .ok_or(MarketplaceError::MathOverflow)?;

    let payout = price
        .checked_sub(fee)
        .ok_or(MarketplaceError::MathOverflow)?;

    // Check if taker has enough lamports for the full price
    if taker.lamports() < price {
        return Err(ProgramError::InsufficientFunds);
    }

    // ---- Transfer payout to seller ----
    invoke(
        &system_instruction::transfer(
            taker.key, 
            maker.key, 
            payout
        ),
        &[
            taker.clone(), 
            maker.clone(), 
            system_program.clone()
        ],
    )?;

    // ---- Transfer fee to marketplace treasury ----
    invoke(
        &system_instruction::transfer(
            taker.key, 
            treasury.key, 
            fee
        ),
        &[
            taker.clone(), 
            treasury.clone(), 
            system_program.clone()
        ],
    )?;

    // ---- Transfer NFT from vault to taker's ATA ----
    // listing PDA signs this transfer as vault's authority
    let signer_seeds: &[&[u8]] = &[
        marketplace.key.as_ref(),
        maker_nft_mint.key.as_ref(),
        &[listing_data.bump],
    ];

    // Use the appropriate transfer instruction based on the token program
    if *token_program.key == TOKEN_PROGRAM_ID {
        invoke_signed(
            &spl_transfer_checked(
                token_program.key,
                vault.key,
                maker_nft_mint.key,
                taker_ata.key,
                listing.key, // vault authority (PDA)
                &[],
                1, // NFT: always 1
                NFT_DECIMALS, // explicitly using 0 for NFTs
            )?,
            &[
                vault.clone(),
                maker_nft_mint.clone(),
                taker_ata.clone(),
                listing.clone(),
                token_program.clone(),
            ],
            &[signer_seeds],
        )?;
    } else {
        invoke_signed(
            &spl_2022_transfer_checked(
                token_program.key,
                vault.key,
                maker_nft_mint.key,
                taker_ata.key,
                listing.key, // vault authority (PDA)
                &[],
                1, // NFT: always 1
                NFT_DECIMALS, // explicitly using 0 for NFTs
            )?,
            &[
                vault.clone(),
                maker_nft_mint.clone(),
                taker_ata.clone(),
                listing.clone(),
                token_program.clone(),
            ],
            &[signer_seeds],
        )?;
    }

    // ---- Close the vault account to reclaim rent for seller ----
    if *token_program.key == TOKEN_PROGRAM_ID {
        invoke_signed(
            &spl_close_account(
                token_program.key,
                vault.key,
                maker.key,   // rent goes to seller
                listing.key, // vault authority
                &[],
            )?,
            &[vault.clone(), maker.clone(), listing.clone(), token_program.clone()],
            &[signer_seeds],
        )?;
    } else {
        invoke_signed(
            &spl_2022_close_account(
                token_program.key,
                vault.key,
                maker.key,   // rent goes to seller
                listing.key, // vault authority
                &[],
            )?,
            &[vault.clone(), maker.clone(), listing.clone(), token_program.clone()],
            &[signer_seeds],
        )?;
    }

    // ---- Close the listing PDA and return rent to seller ----
    invoke_signed(
        &system_instruction::transfer(
            listing.key,
            maker.key,
            listing.lamports(),
        ),
        &[listing.clone(), maker.clone(), system_program.clone()],
        &[signer_seeds],
    )?;

    Ok(())
}