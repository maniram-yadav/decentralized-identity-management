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
            return update_did();
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
        _ => { return Err(ProgramError::InvalidAccountData); }
    }
    
}

fn create_did(program_id : &Pubkey,accounts : &[AccountInfo], doc : DIDDocument ) -> ProgramResult{

    let account_iter = &mut accounts.iter();
    let did_account = next_account_info(account_iter)?;
    let payer = next_account_info(account_iter)?;
    let system_programs = next_account_info(account_iter)?;
    msg!("DID Account : {:?}",did_account);
    msg!("Payer : {:?}",payer);
    msg!("System program : {:?}",system_programs);
    if did_account.owner != program_id {
        msg!("DID account is not owned by program");
        return Err(ProgramError::IncorrectProgramId);
    }


    if *payer.key != doc.owner { 
        msg!("payer is not the owner of account data");
        return Err(ProgramError::InvalidAccountData);
    }

    let mut did_data = DIDDocument::try_from_slice(&did_account.data.borrow())?;
    did_data = doc;
    did_data.serialize(&mut &mut did_account.data.borrow_mut()[..])?;
    Ok(())

}

fn update_did() -> ProgramResult {
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
    use solana_program::clock::Epoch;
    use solana_sdk::{account_info::AccountInfo, pubkey::Pubkey};
    use std::cell::RefCell;

    #[test]
    fn test_create_did() {
        let program_id = Pubkey::new_unique();
        let owner = Pubkey::new_unique();
        let mut lamports1 = 0;
        let mut lamports2 = 0;
        let mut lamports3 = 0;
        let mut data = vec![0; 1024];
        let binding = Pubkey::new_unique();
        let mut data3 = data.clone();
        let did_account = AccountInfo::new(
            &binding,
            false,
            true,
            &mut lamports1,
            &mut data3,
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
        
        let accounts = vec![did_account, payer_account, system_program];
        
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
        
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
    }
}
