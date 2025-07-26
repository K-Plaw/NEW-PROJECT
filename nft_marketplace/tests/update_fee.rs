#[tokio::test]
async fn test_update_fee_ok() {
    use solana_program_test::{processor, ProgramTest};
    use solana_sdk::{
        account::Account,
        pubkey::Pubkey,
        instruction::{AccountMeta, Instruction},
        signature::{Keypair, Signer},
        transaction::Transaction,
        system_program,
    };
    use borsh::BorshSerialize;
    use native_nft_marketplace::{
        entrypoint::ID as PROGRAM_ID,
        instruction::NFTMarketplaceInstructions,
        processor::process_instruction,
    };

    let admin = Keypair::new();
    let name = "codigo-market".to_string();
    let initial_fee = 500u16;
    let updated_fee = 900u16;

    let (marketplace_pda, _) = Pubkey::find_program_address(
        &[b"marketplace", name.as_bytes()],
        &PROGRAM_ID,
    );
    let (treasury_pda, _) = Pubkey::find_program_address(
        &[b"treasury", marketplace_pda.as_ref()],
        &PROGRAM_ID,
    );

    let mut ix_data = vec![];
    NFTMarketplaceInstructions::InitializeMarketplace {
        name: name.clone(),
        fee: initial_fee,
    }
    .serialize(&mut ix_data)
    .unwrap();

    let init_instruction = Instruction {
        program_id: PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(admin.pubkey(), true),
            AccountMeta::new(marketplace_pda, false),
            AccountMeta::new(treasury_pda, false),
            AccountMeta::new_readonly(system_program::id(), false),
            AccountMeta::new_readonly(solana_sdk::sysvar::rent::id(), false),
        ],
        data: ix_data,
    };

    let mut program_test = ProgramTest::new(
        "native_nft_marketplace",
        PROGRAM_ID,
        processor!(process_instruction),
    );

    program_test.add_account(
        admin.pubkey(),
        Account {
            lamports: 10_000_000_000,
            data: vec![],
            owner: system_program::id(),
            executable: false,
            rent_epoch: 0,
        },
    );
    
    let (banks_client, _payer, recent_blockhash) = program_test.start().await;

    let init_tx = Transaction::new_signed_with_payer(
        &[init_instruction],
        Some(&admin.pubkey()),
        &[&admin],
        recent_blockhash,
    );

    banks_client.process_transaction(init_tx).await.unwrap();

    // Now send update_fee instruction
    let mut update_ix_data = vec![];
    NFTMarketplaceInstructions::UpdateFee {
        updated_fee,
    }
    .serialize(&mut update_ix_data)
    .unwrap();

    let update_instruction = Instruction {
        program_id: PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(admin.pubkey(), true),
            AccountMeta::new(marketplace_pda, false),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: update_ix_data,
    };

    let recent_blockhash = banks_client.get_latest_blockhash().await.unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[update_instruction],
        Some(&admin.pubkey()),
        &[&admin],
        recent_blockhash,
    );

    let result = banks_client.process_transaction(tx).await;
    assert!(result.is_ok(), "Update fee tx failed: {:?}", result);
}