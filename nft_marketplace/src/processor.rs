use borsh::BorshDeserialize;
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
};
use crate::instruction::NFTMarketplaceInstructions
;
use crate::instructions::{
    initialize_marketplace::process_initialize_marketplace,
    list_nft::process_list_nft,
    delist_nft::process_delist_nft,
    purchase_nft::process_purchase,
    update_fee::process_update_fee,
};

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = NFTMarketplaceInstructions::try_from_slice(instruction_data)
        .map_err(|_| solana_program::program_error::ProgramError::InvalidInstructionData)?;
    
    match instruction {
        NFTMarketplaceInstructions::InitializeMarketplace { name, fee } => {
            msg!("Instruction: InitializeMarketplace");
            process_initialize_marketplace(program_id, accounts, name, fee)
        },
        NFTMarketplaceInstructions::ListNFT { price, listing_bump } => {
            msg!("Instruction: ListNFT");
            process_list_nft(program_id, accounts, price, listing_bump)
        },
        NFTMarketplaceInstructions::DelistNFT { listing_bump } => {
            msg!("Instruction: DelistNFT");
            process_delist_nft(program_id, accounts, listing_bump)
        },
        NFTMarketplaceInstructions::PurchaseNFT {} => {
            msg!("Instruction: PurchaseNFT");
            process_purchase(program_id, accounts)
        },
        NFTMarketplaceInstructions::UpdateFee { updated_fee } => {
            msg!("Instruction: UpdateFee");
            process_update_fee(program_id, accounts, updated_fee)
        },
    }
}