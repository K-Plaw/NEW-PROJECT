use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    system_instruction,
    program::invoke_signed,
    program_error::ProgramError,
    sysvar::{rent::Rent, Sysvar, SysvarId},
};

use crate::{state::Marketplace, error::MarketplaceError};

/// Maximum fee in basis points (100%)
const MAX_FEE_BPS: u16 = 10_000;

/// Initializes the marketplace.
/// Should only be executed once.
pub fn process_initialize_marketplace(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    name: String,
    fee: u16,
) -> ProgramResult {
    // Validate account count
    if accounts.len() != 5 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    let account_info_iter = &mut accounts.iter();

    let admin = next_account_info(account_info_iter)?;              // Expected to sign the transaction
    let marketplace = next_account_info(account_info_iter)?;        // PDA that will store marketplace state
    let treasury = next_account_info(account_info_iter)?;           // PDA to hold marketplace's earnings
    let system_program = next_account_info(account_info_iter)?;
    let rent_sysvar = next_account_info(account_info_iter)?;        // Rent sysvar account

    // Validate account ownerships
    if system_program.key != &solana_program::system_program::id() {
        return Err(ProgramError::IncorrectProgramId);
    }
    if rent_sysvar.key != &Rent::id() {
        return Err(ProgramError::InvalidAccountData);
    }

    // Only the admin is allowed to initialize the marketplace, so we require a valid signature.
    if !admin.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Enforce constraints on the marketplace name (empty string or too long is not allowed).
    if name.is_empty() || name.len() > 32 {
        return Err(MarketplaceError::NameTooLong.into());
    }

    // Validate fee bounds
    if fee == 0 || fee > MAX_FEE_BPS {
        return Err(MarketplaceError::InvalidFee.into());
    }

    // Derive the PDA for the marketplace using its name as the seed.
    let (marketplace_pda, marketplace_bump) = Pubkey::find_program_address(
        &[b"marketplace", name.as_bytes()],
        program_id,
    );

    // Treasury PDA is derived from the marketplace PDA.
    let (treasury_pda, treasury_bump) = Pubkey::find_program_address(
        &[b"treasury", marketplace_pda.as_ref()],
        program_id,
    );

    // Make sure the marketplace account passed in is the correct PDA.
    if *marketplace.key != marketplace_pda {
        return Err(ProgramError::InvalidSeeds);
    }

    // Also verify that the treasury PDA is what we expect.
    if *treasury.key != treasury_pda {
        return Err(ProgramError::InvalidSeeds);
    }

    // Check if marketplace is already initialized by checking data length
    if marketplace.data_len() > 0 {
        return Err(MarketplaceError::AlreadyInitialized.into());
    }

    // Check if treasury is already initialized
    if treasury.data_len() > 0 {
        return Err(MarketplaceError::AlreadyInitialized.into());
    }

    // Calculate how much lamports are needed to make the new marketplace account rent-exempt.
    let rent = Rent::get()?;
    let marketplace_size = Marketplace::size(&name);
    let marketplace_rent = rent.minimum_balance(marketplace_size);
    let treasury_rent = rent.minimum_balance(marketplace_size);

    // Create the marketplace account with proper seeds
    let marketplace_seeds = &[b"marketplace" as &[u8], name.as_bytes(), &[marketplace_bump]];
    let marketplace_signer_seeds = &[&marketplace_seeds[..]];

    // Create the marketplace account, allocating space and funding it with rent.
    invoke_signed(
        &system_instruction::create_account(
            admin.key,                     // funding from admin
            marketplace.key,               // account being created
            marketplace_rent,              // amount to fund for rent exemption
            marketplace_size as u64,                   // size of the account
            program_id,                    // program owns the account
        ),
        &[admin.clone(), marketplace.clone(), system_program.clone()],
        marketplace_signer_seeds,
    )?;

    // Create the treasury account with proper seeds
    let treasury_seeds = &[b"treasury" as &[u8], marketplace_pda.as_ref(), &[treasury_bump]];
    let treasury_signer_seeds = &[&treasury_seeds[..]];

    // Create the treasury account
    invoke_signed(
        &system_instruction::create_account(
            admin.key,
            treasury.key,
            treasury_rent,
            marketplace_size as u64,
            program_id,
        ),
        &[admin.clone(), treasury.clone(), system_program.clone()],
        treasury_signer_seeds,
    )?;

    // Verify both accounts are rent exempt
    if !rent.is_exempt(marketplace.lamports(), marketplace_size) {
        return Err(ProgramError::AccountNotRentExempt);
    }
    if !rent.is_exempt(treasury.lamports(), marketplace_size) {
        return Err(ProgramError::AccountNotRentExempt);
    }

    // Initialize the marketplace state and serialize it into the newly created account.
    let marketplace_state = Marketplace::new(
        *admin.key,         // set admin as authority
        fee,                // initial fee in bps
        marketplace_bump,
        *treasury.key,      // treasury PDA
        treasury_bump,
        name,               // marketplace name
    )?;

    // Write discriminator first (8 bytes)
    let mut data = marketplace.data.borrow_mut();
    let discriminator = Marketplace::get_discriminator();
    data[0..8].copy_from_slice(&discriminator[..8]);

    // Then write the data
    marketplace_state.serialize(&mut &mut data[8..])?;

    Ok(())
}