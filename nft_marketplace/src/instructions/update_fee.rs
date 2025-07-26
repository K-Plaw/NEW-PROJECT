use solana_program::{
    account_info::{next_account_info, AccountInfo},
    program_error::ProgramError,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    system_program,
};

use borsh::{BorshDeserialize, BorshSerialize};

use crate::{state::Marketplace, error::MarketplaceError};

/// Max fee allowed: 100% in basis points = 10_000
const MAX_FEE_BPS: u16 = 10_000;

/// Updates the `fee_bps` of a Marketplace account.
/// Can only be executed by the marketplace authority/admin.
pub fn process_update_fee(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    updated_fee: u16,
) -> ProgramResult {
    // Validate account count
    if accounts.len() != 3 {  // Removed token_program as it was unused
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    let account_info_iter = &mut accounts.iter();

    let admin = next_account_info(account_info_iter)?;          // Admin must sign
    let marketplace = next_account_info(account_info_iter)?;    // PDA of the marketplace
    let system_program = next_account_info(account_info_iter)?; // System program

    // Validate program IDs
    if system_program.key != &system_program::id() {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Check signer
    if !admin.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Validate marketplace account
    if !marketplace.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }
    
    if marketplace.owner != program_id {
        return Err(ProgramError::IllegalOwner);
    }
    
    // Get marketplace data and verify discriminator
    let data = marketplace.data.borrow();
    if data[0..8] != Marketplace::get_discriminator()[..8] {
        return Err(ProgramError::InvalidAccountData);
    }

    // Deserialize Marketplace data
    let mut marketplace_data = Marketplace::try_from_slice(&data[8..])?;

    // Verify marketplace PDA
    let (marketplace_pda, marketplace_bump) = Pubkey::find_program_address(
        &[b"marketplace", marketplace_data.name.as_bytes()],
        program_id,
    );
    if marketplace_pda != *marketplace.key {
        return Err(MarketplaceError::InvalidMarketplaceAccount.into());
    }
    
    // Verify the stored bump matches the derived bump
    if marketplace_data.marketplace_bump != marketplace_bump {
        return Err(MarketplaceError::InvalidMarketplaceAccount.into());
    }

    // Authorization check
    if *admin.key != marketplace_data.authority {
        return Err(ProgramError::IllegalOwner);
    }

    // Fee validation - enhanced to catch all invalid cases in one check
    if updated_fee == 0 || 
       updated_fee > MAX_FEE_BPS || 
       updated_fee == marketplace_data.fee_bps {
        return Err(MarketplaceError::InvalidFee.into());
    }

    // Update fee
    marketplace_data.fee_bps = updated_fee;

    // Drop the immutable borrow before getting a mutable borrow
    drop(data);

    // Persist updated data
    let mut data = marketplace.data.borrow_mut();
    marketplace_data.serialize(&mut &mut data[8..])?;

    Ok(())
}