use solana_program::{
    pubkey::Pubkey,
};

use crate::didpublickey::DIDPublicKey;
use crate::didservice::DIDService;

use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize,BorshDeserialize,Debug)]
pub struct DIDDocument {
    pub version : u8,
    pub owner : Pubkey,
    pub public_keys : Vec<DIDPublicKey>,
    pub services : Vec<DIDService>,
    pub created : i64,
    pub updated : i64,

}
 