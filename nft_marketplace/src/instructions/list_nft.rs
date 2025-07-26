use borsh::{BorshDeserialize, BorshSerialize};

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
    program_pack::Pack
};

use spl_associated_token_account::{
    get_associated_token_address,
    instruction::create_associated_token_account,
};

use spl_token::{
    instruction::transfer_checked as spl_transfer_checked,
    state::Account as TokenAccount,
    ID as TOKEN_PROGRAM_ID,
    state::AccountState,
};

use spl_token_2022::{
    instruction::transfer_checked as spl_2022_transfer_checked,
    ID as TOKEN_2022_PROGRAM_ID,
};

use crate::{state::{Listing, Marketplace}, error::MarketplaceError};

/// Instruction: List NFT on the marketplace
pub fn process_list_nft(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    price: u64,
    listing_bump: u8,
) -> ProgramResult {
    // Validate account count
    if accounts.len() != 11 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    let account_info_iter = &mut accounts.iter();

    // signer, writable
    let maker = next_account_info(account_info_iter)?;

    // read-only PDA, not initialized in this instruction
    let marketplace = next_account_info(account_info_iter)?;

    // read-only: NFT mint
    let maker_nft_mint = next_account_info(account_info_iter)?;

    // writable: maker's NFT token account
    let maker_nft_ata = next_account_info(account_info_iter)?;

    // writable: vault account (ATA of the NFT mint owned by the listing PDA)
    let vault = next_account_info(account_info_iter)?;

    // writable PDA: Listing account being initialized
    let listing = next_account_info(account_info_iter)?;

    // read-only: metadata account (Metaplex), verified only
    let metadata = next_account_info(account_info_iter)?;

    // read-only: associated token program
    let ata_program = next_account_info(account_info_iter)?;

    // read-only: system program
    let system_program = next_account_info(account_info_iter)?;

    // read-only: token program (either SPL Token or SPL Token 2022)
    let token_program = next_account_info(account_info_iter)?;

    // Get rent sysvar
    let rent = Rent::get()?;

    // === Critical Account Validations ===

    // Validate program IDs
    if system_program.key != &solana_program::system_program::id() {
        return Err(ProgramError::IncorrectProgramId);
    }
    if ata_program.key != &spl_associated_token_account::id() {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Validate token program is either SPL Token or SPL Token 2022
    if *token_program.key != TOKEN_PROGRAM_ID && *token_program.key != TOKEN_2022_PROGRAM_ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Validate token accounts are owned by the correct token program
    if maker_nft_ata.owner != token_program.key {
        return Err(ProgramError::IllegalOwner);
    }

    // Check signer
    if !maker.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Validate price
    if price == 0 {
        return Err(MarketplaceError::InvalidPrice.into());
    }

    // Verify marketplace PDA and ownership
    let marketplace_data = Marketplace::try_from_slice(&marketplace.data.borrow())?;
    let (marketplace_pda, _) = Pubkey::find_program_address(
        &[b"marketplace", marketplace_data.name.as_bytes()],
        program_id,
    );
    if marketplace_pda != *marketplace.key {
        return Err(MarketplaceError::InvalidMarketplaceAccount.into());
    }
    if marketplace.owner != program_id {
        return Err(ProgramError::IllegalOwner);
    }

    // Verify maker's ATA
    let expected_maker_ata = get_associated_token_address(maker.key, maker_nft_mint.key);
    if *maker_nft_ata.key != expected_maker_ata {
        return Err(MarketplaceError::InvalidTokenAccount.into());
    }

    // Verify NFT token state
    let maker_token_data = TokenAccount::unpack(&maker_nft_ata.try_borrow_data()?)?;
    if maker_token_data.amount != 1 {
        return Err(MarketplaceError::InvalidTokenAmount.into());
    }
    if maker_token_data.mint != *maker_nft_mint.key {
        return Err(MarketplaceError::InvalidTokenMint.into());
    }
    if maker_token_data.state != AccountState::Initialized {
        return Err(MarketplaceError::InvalidTokenState.into());
    }

    // Verify listing PDA derivation
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

    // Check if listing already exists
    if listing.data_len() > 0 {
        return Err(MarketplaceError::ListingAlreadyExists.into());
    }

    // Verify vault ATA derivation
    let expected_vault = get_associated_token_address(listing.key, maker_nft_mint.key);
    if *vault.key != expected_vault {
        return Err(ProgramError::InvalidArgument);
    }

    // === Create the Listing PDA account ===
    let listing_space = Listing::SIZE;
    let listing_rent = rent.minimum_balance(listing_space);

    let seeds = &[
        marketplace.key.as_ref(),
        maker_nft_mint.key.as_ref(),
        &[listing_bump],
    ];
    let signer_seeds = &[&seeds[..]];

    invoke_signed(
        &system_instruction::create_account(
            maker.key,
            listing.key,
            listing_rent,
            listing_space as u64,
            program_id,
        ),
        &[
            maker.clone(),
            listing.clone(),
            system_program.clone(),
        ],
        signer_seeds,
    )?;

    // Verify rent exemption after account creation
    if !rent.is_exempt(listing.lamports(), listing_space) {
        return Err(ProgramError::AccountNotRentExempt);
    }

    // === Serialize and write Listing account state ===
    let listing_data = Listing::new(
        *maker.key,
        *maker_nft_mint.key,
        price,
        *metadata.key,
        listing_bump,
        *token_program.key,
    );

    // Write discriminator first (8 bytes)
    let mut data = listing.data.borrow_mut();
    let discriminator = Listing::get_discriminator();
    data[0..8].copy_from_slice(&discriminator[..8]);
    // Then write the data
    listing_data.serialize(&mut &mut data[8..])?;

    // === Check if vault already exists ===
    let vault_exists = vault.data_len() > 0;

    // === Create Vault ATA (owned by Listing PDA) if it doesn't exist ===
    if !vault_exists {
        invoke(
            &create_associated_token_account(
                maker.key,            // funder
                listing.key,          // owner of the ATA
                maker_nft_mint.key,
                token_program.key,
            ),
            &[
                maker.clone(),
                vault.clone(),
                listing.clone(),
                maker_nft_mint.clone(),
                system_program.clone(),
                token_program.clone(),
                ata_program.clone(),
            ],
        )?;

        // Verify vault ATA is rent exempt
        const TOKEN_ACCOUNT_SIZE: usize = 165;
        if !rent.is_exempt(vault.lamports(), TOKEN_ACCOUNT_SIZE) {
            return Err(ProgramError::AccountNotRentExempt);
        }
    } else {
        // If vault exists, verify it belongs to the listing
        let vault_data = TokenAccount::unpack(&vault.try_borrow_data()?)?;
        if vault_data.owner != *listing.key {
            return Err(MarketplaceError::InvalidVaultOwner.into());
        }
        if vault_data.mint != *maker_nft_mint.key {
            return Err(MarketplaceError::InvalidTokenMint.into());
        }
    }

    // === Transfer the NFT from maker to vault ===
    // Use the appropriate transfer instruction based on the token program
    if *token_program.key == TOKEN_PROGRAM_ID {
        invoke(
            &spl_transfer_checked(
                token_program.key,
                maker_nft_ata.key,
                maker_nft_mint.key,
                vault.key,
                maker.key,
                &[], // no multisig
                1,   // amount
                0,   // decimals (NFT)
            )?,
            &[
                maker_nft_ata.clone(),
                maker_nft_mint.clone(),
                vault.clone(),
                maker.clone(),
                token_program.clone(),
            ],
        )?;
    } else {
        invoke(
            &spl_2022_transfer_checked(
                token_program.key,
                maker_nft_ata.key,
                maker_nft_mint.key,
                vault.key,
                maker.key,
                &[], // no multisig
                1,   // amount
                0,   // decimals (NFT)
            )?,
            &[
                maker_nft_ata.clone(),
                maker_nft_mint.clone(),
                vault.clone(),
                maker.clone(),
                token_program.clone(),
            ],
        )?;
    }

    // Verify the vault now has the NFT
    let vault_data = TokenAccount::unpack(&vault.try_borrow_data()?)?;
    if vault_data.amount != 1 {
        return Err(MarketplaceError::InvalidTokenAmount.into());
    }

    Ok(())
}