use crate::common::*;

/* 
    =========================================================================================
    =========== AES encryption-related key and initialization vector information. ===========
    =========================================================================================
*/
#[derive(Deserialize, Debug, Getters, Setters, MutGetters, Default)]
#[getset(get = "pub")]
pub struct AesInfosDTO {
    pub aes_key: Vec<u8>,
    pub aes_iv: Vec<u8>
}

impl AesInfosDTO {
    pub fn new(aes_key_str: &str, aes_iv_str: &str) -> Self {
        AesInfosDTO {
            aes_key: aes_key_str.as_bytes().to_vec(),
            aes_iv: aes_iv_str.as_bytes().to_vec(),
        }
    }
}