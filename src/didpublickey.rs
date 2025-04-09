use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize,BorshDeserialize,Debug)]

pub struct DIDPublicKey {
    pub id : u8,
    pub key_type : String,
    pub public_keys : Vec<u8>,
    pub controller : String,

}
 