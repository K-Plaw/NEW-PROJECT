use borsh::BorshDeserialize;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    program_pack::Pack
};
use spl_token::{
    instruction::transfer_checked as spl_transfer_checked,
    state::Account as TokenAccount,
    ID as TOKEN_PROGRAM_ID,
};

use spl_token_2022::{
    instruction::transfer_checked as spl_2022_transfer_checked,
    ID as TOKEN_2022_PROGRAM_ID,
};
use crate::{
    state::{Listing, Marketplace},
    error::MarketplaceError,
};

/// Instruction: Delist NFT on the marketplace
pub fn process_delist_nft(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    listing_bump: u8,
) -> ProgramResult {
    // Validate account count
    if accounts.len() != 8 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    let account_info_iter = &mut accounts.iter();

    // signer, writable
    let maker = next_account_info(account_info_iter)?;

    // read-only PDA (not modified here)
    let marketplace = next_account_info(account_info_iter)?;

    // read-only NFT mint
    let maker_nft_mint = next_account_info(account_info_iter)?;

    // writable: maker's NFT associated token account
    let maker_nft_ata = next_account_info(account_info_iter)?;

    // writable: vault (ATA of NFT mint owned by listing PDA)
    let vault = next_account_info(account_info_iter)?;

    // writable PDA, will be closed after this instruction
    let listing = next_account_info(account_info_iter)?;

    // read-only: SPL token program
    let token_program = next_account_info(account_info_iter)?;

    // === Validate accounts ===

    // Check signer
    if !maker.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Verify program ownership
    if listing.owner != program_id || marketplace.owner != program_id {
        return Err(ProgramError::IllegalOwner);
    }

    // Validate token program is either SPL Token or SPL Token 2022
    if *token_program.key != TOKEN_PROGRAM_ID && *token_program.key != TOKEN_2022_PROGRAM_ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    //  Deserialize the marketplace to verify the marketplace PDA
    let marketplace_data = Marketplace::try_from_slice(&marketplace.data.borrow())?;
    let (marketplace_pda, _) = Pubkey::find_program_address(
        &[b"marketplace", marketplace_data.name.as_bytes()],
        program_id,
    );
    if marketplace_pda != *marketplace.key {
        return Err(MarketplaceError::InvalidMarketplaceAccount.into());
    }

    // Verify listing PDA
    let (listing_pda, expected_bump) = Pubkey::find_program_address(
        &[
            marketplace.key.as_ref(),
            maker_nft_mint.key.as_ref(),
        ],
        program_id,
    );
    if listing_pda != *listing.key || listing_bump != expected_bump {
        return Err(MarketplaceError::InvalidListingAccount.into());
    }

    // Verify listing account discriminator
    let data = listing.data.borrow();
    if data[0..8] != Listing::get_discriminator()[..8] {
        return Err(ProgramError::InvalidAccountData);
    }

    // Deserialize the listing to verify maker is the owner
    let listing_data = Listing::try_from_slice(&data[8..])?;
    if listing_data.maker != *maker.key {
        return Err(MarketplaceError::InvalidListingOwner.into());
    }

    // Verify NFT mint matches
    if listing_data.nft_mint != *maker_nft_mint.key {
        return Err(MarketplaceError::InvalidTokenMint.into());
    }

    // Verify vault ownership
    let vault_data = TokenAccount::unpack(&vault.try_borrow_data()?)?;
    if vault_data.owner != *listing.key {
        return Err(MarketplaceError::InvalidVaultOwner.into());
    }

    // Verify token state
    if vault_data.amount != 1 {
        return Err(MarketplaceError::InvalidTokenAmount.into());
    }
    if vault_data.mint != *maker_nft_mint.key {
        return Err(MarketplaceError::InvalidTokenMint.into());
    }

    // Verify the token program ID
    if *token_program.key != TOKEN_PROGRAM_ID && *token_program.key != TOKEN_2022_PROGRAM_ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Verify maker's ATA
    let expected_maker_ata = spl_associated_token_account::get_associated_token_address(
        maker.key, 
        maker_nft_mint.key
    );
    if *maker_nft_ata.key != expected_maker_ata {
        return Err(MarketplaceError::InvalidTokenAccount.into());
    }

    // === Transfer NFT from vault back to maker ===

    // Derive signer seeds for listing PDA (used as vault authority)
    let seeds = &[
        marketplace.key.as_ref(),
        maker_nft_mint.key.as_ref(),
        &[listing_bump],
    ];
    let signer_seeds = &[&seeds[..]];

    // Choose the appropriate transfer instruction based on the token program
    let transfer_ix = if *token_program.key == TOKEN_PROGRAM_ID {
        spl_transfer_checked(
            token_program.key,
            vault.key,
            maker_nft_mint.key,
            maker_nft_ata.key,
            listing.key,
            &[],
            1, // transfer 1 NFT
            0, // decimals = 0 for NFTs
        )?
    } else {
        spl_2022_transfer_checked(
            token_program.key,
            vault.key,
            maker_nft_mint.key,
            maker_nft_ata.key,
            listing.key,
            &[],
            1, // transfer 1 NFT
            0, // decimals = 0 for NFTs
        )?
    };

    invoke_signed(
        &transfer_ix,
        &[
            vault.clone(),
            maker_nft_mint.clone(),
            maker_nft_ata.clone(),
            listing.clone(),
            token_program.clone(),
        ],
        signer_seeds,
    )?;

    // === Close the vault ===

    // Choose the appropriate close account instruction based on the token program
    let close_ix = if *token_program.key == TOKEN_PROGRAM_ID {
        spl_token::instruction::close_account(
            token_program.key,
            vault.key,
            maker.key,
            listing.key,
            &[],
        )?
    } else {
        spl_token_2022::instruction::close_account(
            token_program.key,
            vault.key,
            maker.key,
            listing.key,
            &[],
        )?
    };

    invoke_signed(
        &close_ix,
        &[
            vault.clone(),
            maker.clone(),
            listing.clone(),
            token_program.clone(),
        ],
        signer_seeds,
    )?;

    // Close the listing account - transfer lamports back to maker
    let dest_starting_lamports = maker.lamports();
    **maker.lamports.borrow_mut() = dest_starting_lamports
        .checked_add(listing.lamports())
        .ok_or(MarketplaceError::MathOverflow)?;
    **listing.lamports.borrow_mut() = 0;
    
    // Clear the listing data
    let mut listing_data = listing.try_borrow_mut_data()?;
    for byte in listing_data.iter_mut() {
        *byte = 0;
    }

    Ok(())
}