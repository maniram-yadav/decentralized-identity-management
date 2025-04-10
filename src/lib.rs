extern crate solana_program;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{clock::Clock, Sysvar},
};
use borsh::{BorshDeserialize, BorshSerialize};

mod diddocument;
mod didinstruction;
mod didpublickey;
mod didservice;

use didinstruction::DIDInstruction;
use crate::diddocument::DIDDocument;

entrypoint!(process_instruction);
pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {

    msg!("Processing Instruction");
    let instruction = DIDInstruction::try_from_slice(_instruction_data)?;
    match instruction {

        DIDInstruction::CreateDID{ initial_doc } => {
            msg!{"CreateDID"};
            return create_did(_program_id, _accounts, initial_doc);
        },
        DIDInstruction::UpdateDID{ updated_doc } => {
            msg!{"UpdateDID"};
            return update_did(_program_id, _accounts, updated_doc);
        },
        DIDInstruction::AddPublicKey{ public_key } => {
            msg!{"AddPublicKey"};
            return add_public_key();
        },
        DIDInstruction::RemovePublicKey{ key_id } => {
            msg!{"RemovePublicKey"};
            return remove_public_key();
        },
        DIDInstruction::AddService{ service } => {
            msg!{"AddService"};
            return add_service();
        },
        DIDInstruction::RemoveService{ service_id } => {
            msg!{"RemoveService"};
            return remove_service();
        },
    }
    
}

fn create_did(program_id : &Pubkey,accounts : &[AccountInfo], doc : DIDDocument ) -> ProgramResult{

    let account_iter = &mut accounts.iter();
    let did_account = next_account_info(account_iter)?;
    let payer = next_account_info(account_iter)?;
    let system_programs = next_account_info(account_iter)?;
    msg!("Program : {:?}",program_id);
    msg!("DID Account : {:?}",did_account);
    msg!("Payer : {:?}",payer);
    msg!("System program : {:?}",system_programs);

    if did_account.owner != program_id {
        msg!("DID account is not owned by program");
        return Err(ProgramError::IncorrectProgramId);
    }

    msg!("Comparing payer with owner");
    if *payer.key != doc.owner { 
        msg!("payer is not the owner of account data");
        return Err(ProgramError::InvalidAccountData);
    }
    
    let mut did_data = DIDDocument::try_from_slice(&did_account.data.borrow())?;
    did_data = doc;
    msg!("Serializing Account data");
    did_data.serialize(&mut &mut did_account.data.borrow_mut()[..])?;

    Ok(())

}

fn update_did(program_id : &Pubkey,
        accounts : &[AccountInfo],
        updated_doc:DIDDocument) -> ProgramResult {
          
            let account_iter = &mut accounts.iter();
            let did_account = next_account_info(account_iter)?;
            let owner = next_account_info(account_iter)?;
            let clock = Clock::get()?;

            if did_account.owner != program_id {
                msg!("DID account is not owned by the program");
                return Err(ProgramError::IncorrectProgramId);
            }
             
            if !owner.is_signer {
                msg!("Owner did not sign the transaction");
                return Err(ProgramError::MissingRequiredSignature);
            }
            
            let mut current_doc = DIDDocument::try_from_slice(&did_account.data.borrow())?;
            if current_doc.owner != *owner.key {
                
                msg!("Signer is not the owner of the DID");
                return Err(ProgramError::InvalidAccountData);
            }

            if updated_doc.version <= current_doc.version {
                msg!("Version must increment");
                return Err(ProgramError::InvalidAccountData);
            }
            current_doc = updated_doc;
            current_doc.updated = clock.unix_timestamp;
            current_doc.serialize(&mut &mut did_account.data.borrow_mut()[..])?;
            Ok(())
}


fn add_public_key() -> ProgramResult {
    Ok(())
}



fn remove_public_key() -> ProgramResult {
    Ok(())
}


fn add_service() -> ProgramResult {
    Ok(())
}


fn remove_service() -> ProgramResult {
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{self, Write}; 
    use solana_program::clock::Epoch;
    use solana_sdk::{account_info::AccountInfo, pubkey::Pubkey};
    use std::cell::RefCell;

    // use solana_program::log::sol_logger_setup;

    use crate::diddocument::DIDDocument;
    use crate::didpublickey::DIDPublicKey;
    use crate::didservice::DIDService;


    fn get_diddocument(owner : &Pubkey) -> DIDDocument {

        DIDDocument {
            version: 1,
            owner: *owner,
            public_keys: vec![
                DIDPublicKey {
                    id: 1,
                    key_type: "Ed25519VerificationKey2018".to_string(),
                    public_keys: vec![
                        1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
                        11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
                        21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
                        31, 32
                    ],
                    controller: "did:solana:fixed-owner".to_string(),
                }
            ],
            services: vec![
                DIDService {
                    id: "auth-service".to_string(),
                    service_type: "LinkedDomains".to_string(),
                    service_endpoint: "https://fixed.example.com/auth".to_string(),
                },
                DIDService {
                    id: "storage-service".to_string(),
                    service_type: "CredentialRepository".to_string(),
                    service_endpoint: "https://fixed.example.com/storage".to_string(),
                }
            ],
            created: 1_650_000_000, // Fixed timestamp: July 2022
            updated: 1_650_000_000,
        }
    
    }


    #[test]
    fn test_create_did() {
        
        // sol_logger_setup();

        let program_id = Pubkey::new_unique();
        let owner = Pubkey::new_unique();
        let mut lamports1 = 1000000;
        let mut lamports2 = 1000000;
        let mut lamports3 = 1000000;
        
        let mut data = vec![0; 1024];
        // data.push(2);

        let binding = Pubkey::new_unique();
        let mut data3 = data.clone();
        
        
        // Calculate required lamports for rent exemption
        // let lamports = rent.minimum_balance(serialized.len());
        

        let did_data = get_diddocument(&owner);
        let serialized = did_data.try_to_vec().unwrap();
        let mut did_data = vec![0; serialized.len()];
        did_data.copy_from_slice(&serialized);
    
        
        let did_account = AccountInfo::new(
            &binding,
            false,
            true,
            &mut lamports1,
            &mut did_data,
            &program_id,
            false,
            Epoch::default(),
        );
        let mut data1 = data.clone();
        let payer_account = AccountInfo::new(
            &owner,
            true,
            false,
            &mut lamports2,
            &mut data1,
            &program_id,
            false,
            Epoch::default(),
        );
        let mut data2 = data.clone();
        let binding = solana_program::system_program::id();
        let system_program = AccountInfo::new(
            &binding,
            false,
            false,
            &mut lamports3,
            &mut data2,
            &program_id,
            false,
            Epoch::default(),
        );
        
        let accounts = vec![did_account.clone(), payer_account, system_program];
        
        let initial_doc = DIDDocument {
            version: 1,
            owner,
            public_keys: vec![],
            services: vec![],
            created: 0,
            updated: 0,
        };
        
        let instruction = DIDInstruction::CreateDID { initial_doc };
        let mut instruction_data = vec![];
        instruction.serialize(&mut instruction_data).unwrap();
        
        

        let result = process_instruction(&program_id, &accounts, &instruction_data);
        
        match &result {
            Ok(msg) => msg!("{:?}",msg),
            Err(er) => msg!("{:?}",er),
        }

        std::io::stdout().flush().unwrap();

        assert!(result.is_ok());
    

    }
}
