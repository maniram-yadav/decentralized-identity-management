use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize,BorshDeserialize,Debug)]
pub struct DIDService {
    pub id : String,
    pub service_type : String,
    pub service_endpoint : String,    
}
 