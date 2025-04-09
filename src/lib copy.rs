extern crate solana_program;
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};

entrypoint!(process_instruction);

// decentralized_identity_management.so
// Program Id: 91tVFdZmEJDdvG5HfDgQegfEzhhYYWYCXMcY7r86fjvk

// Signature: 4KWLNUR1zXJULmr3bhT9Lbc5RaFrxvm1aC12Z23eDuH9XxzoDAifpArxXBkSU3Y9eouy8zmhhYuxoVQRNWC41hjJ