#[cfg(test)]
mod tests {
    use solana_program_test::{processor, ProgramTest};
    use solana_sdk::{
        account::Account,
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        signature::{Keypair, Signer},
        transaction::Transaction,
        system_program,
    };

    use borsh::BorshSerialize;

    use native_nft_marketplace::{
        instruction::NFTMarketplaceInstructions,
        processor::process_instruction,
        entrypoint::ID as PROGRAM_ID
    };

    #[tokio::test]
    async fn test_initialize_marketplace_ok() {
        // Set up test environment
        let admin = Keypair::new();
        let name = "codigo-market".to_string();
        let fee = 500u16; // 5%

        // Derive PDAs
        let (marketplace_pda, _) = Pubkey::find_program_address(
            &[b"marketplace", name.as_bytes()],
            &PROGRAM_ID,
        );
        let (treasury_pda, _) = Pubkey::find_program_address(
            &[b"treasury", marketplace_pda.as_ref()],
            &PROGRAM_ID,
        );

        // Prepare instruction data
        let mut ix_data = vec![];
        NFTMarketplaceInstructions::InitializeMarketplace {
            name: name.clone(),
            fee,
        }
        .serialize(&mut ix_data)
        .unwrap();
        
        

        // Construct instruction
        let instruction = Instruction {
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

        // Create ProgramTest environment
        let mut program_test = ProgramTest::new(
            "native_nft_marketplace",
            PROGRAM_ID,
            processor!(process_instruction),
        );

        // Add admin with some lamports
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

        let tx = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&admin.pubkey()),
            &[&admin],
            recent_blockhash,
        );

        let result = banks_client.process_transaction(tx).await;
        assert!(result.is_ok(), "Transaction failed: {:?}", result);
    }

    #[tokio::test]
    async fn test_initialize_marketplace_invalid_fee() {

        let admin = Keypair::new();
        let name = "codigo-market-invalid-fee".to_string();
        let fee = 20_000u16; // invalid: > 100%

        // Derive PDAs
        let (marketplace_pda, _) = Pubkey::find_program_address(
            &[b"marketplace", name.as_bytes()],
            &PROGRAM_ID,
        );
        let (treasury_pda, _) = Pubkey::find_program_address(
            &[b"treasury", marketplace_pda.as_ref()],
            &PROGRAM_ID,
        );

        // Prepare instruction data
        let mut ix_data = vec![];
        NFTMarketplaceInstructions::InitializeMarketplace {
            name: name.clone(),
            fee,
        }
        .serialize(&mut ix_data)
        .unwrap();

        // Construct instruction
        let instruction = Instruction {
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

        let tx = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&admin.pubkey()),
            &[&admin],
            recent_blockhash,
        );

        let result = banks_client.process_transaction(tx).await;

        assert!(
            result.is_err(),
            "Transaction should fail with InvalidFee"
        );
    }

    #[tokio::test]
    async fn test_initialize_marketplace_name_empty() {
        // Setup
        let admin = Keypair::new();
        let name = "".to_string(); // 0 length = empty
        let fee = 500u16;

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
            fee,
        }
        .serialize(&mut ix_data)
        .unwrap();

        let instruction = Instruction {
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

        let tx = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&admin.pubkey()),
            &[&admin],
            recent_blockhash,
        );

        let result = banks_client.process_transaction(tx).await;

        assert!(
            result.is_err(),
            "Transaction should fail with NameTooLong"
        );
    }
}