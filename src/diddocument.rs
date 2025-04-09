pub struct DIDDocument {
    pub version : u8,
    pub owner : Pubkey,
    pub public_keys : Vec<DIDPublicKey>,
    pub services : Vec<DIDServices>,
    pub created : i64,
    pub updated : i64,

}
 