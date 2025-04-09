use borsh::{BorshDeserialize, BorshSerialize};
use crate::diddocument::DIDDocument;
use crate::didpublickey::DIDPublicKey;
use crate::didservice::DIDService;

#[derive(BorshSerialize,BorshDeserialize,Debug)]
pub enum DIDInstruction {
    CreateDID {
        initial_doc : DIDDocument,
    },
    UpdateDID {
        updated_doc : DIDDocument,
    },
    AddPublicKey {
        public_key : DIDPublicKey,
    },
    RemovePublicKey {
        key_id : String,
    },
    AddService {
        service : DIDService,
    },
    RemoveService {
        service_id : String,
    },
}
 