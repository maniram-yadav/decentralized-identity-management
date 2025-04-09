extern crate solana_program;
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};
use borsh::{BorshDeserialize, BorshSerialize};

mod diddocument;
mod didinstruction;
mod didpublickey;
mod didservice;
use diddocument::DIDDocument;
use didinstruction::DIDInstruction;
// use crate::didinstruction::DIDInstruction;

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    
    let instruction = DIDInstruction::try_from_slice(_instruction_data)?;
    match instruction {
        DIDInstruction::CreateDID{ initial_doc } => {
            msg!{"CreateDID"};
        },
        DIDInstruction::UpdateDID{ updated_doc } => {
            msg!{"UpdateDID"};
        },
        DIDInstruction::AddPublicKey{ public_key } => {
            msg!{"AddPublicKey"};
        },
        DIDInstruction::RemovePublicKey{ key_id } => {
            msg!{"RemovePublicKey"};
        },
        DIDInstruction::AddService{ service } => {
            msg!{"AddService"};
        },
        DIDInstruction::RemoveService{ service_id } => {
            msg!{"RemoveService"};
        },
    }
    msg!("Hello, world!");
    Ok(())
}

#[cfg(test)]
mod test {
    use solana_program_test::*;
    use solana_sdk::{
        instruction::Instruction, pubkey::Pubkey, signature::Signer, transaction::Transaction,
    };

    #[tokio::test]
    async fn test_hello_world() {
        let program_id = Pubkey::new_unique();
        let mut program_test = ProgramTest::default();
        program_test.add_program("decentralized_identity_management", program_id, None);
        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;
        // Create instruction
        let instruction = Instruction {
            program_id,
            accounts: vec![],
            data: vec![],
        };
        // Create transaction with instruction
        let mut transaction = Transaction::new_with_payer(&[instruction], Some(&payer.pubkey()));

        // Sign transaction
        transaction.sign(&[&payer], recent_blockhash);

        let transaction_result = banks_client.process_transaction(transaction).await;
        assert!(transaction_result.is_ok());
    }
}

// #[cfg(test)]
// mod test {
//     use solana_program_test::*;
//     use solana_sdk::{
//         instruction::Instruction, pubkey::Pubkey, signature::Signer, transaction::Transaction,
//     };

//     #[tokio::test]
//     async fn test_hello_world() {
//         let program_id = Pubkey::new_unique();
//         let mut program_test = ProgramTest::default();
//         program_test.add_program("hello_world", program_id, None);
//         let (mut banks_client, payer, recent_blockhash) = program_test.start().await;
//         // Create instruction
//         let instruction = Instruction {
//             program_id,
//             accounts: vec![],
//             data: vec![],
//         };
//         // Create transaction with instruction
//         let mut transaction = Transaction::new_with_payer(&[instruction], Some(&payer.pubkey()));

//         // Sign transaction
//         transaction.sign(&[&payer], recent_blockhash);

//         let transaction_result = banks_client.process_transaction(transaction).await;
//         assert!(transaction_result.is_ok());
//     }
// }