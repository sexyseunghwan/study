use crate::common::*;

use crate::dto::aes_dtos::AesInfosDTO;

//use crate::utils_modules::parsing_utils::*;

/*
    Function that decrypts an encrypted string. (AES-256)    
*/
pub fn decrypt(ciphertext: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, anyhow::Error> {
    
    let cipher = Cipher::aes_256_cbc();
    let mut crypter = Crypter::new(cipher, Mode::Decrypt, key, Some(iv))?;
    let mut plaintext = vec![0; ciphertext.len() + cipher.block_size()];
    let count = crypter.update(ciphertext, &mut plaintext)?;
    let rest = crypter.finalize(&mut plaintext[count..])?;
    plaintext.truncate(count + rest);

    Ok(plaintext)
}


/*
    Function that converts the Initialization Vector and KEY information of aes into a structure.
*/
pub fn get_aes_infos() -> Result<AesInfosDTO, anyhow::Error> {
    
    let aes_key: String = env::var("AES_KEY").expect("'AES_KEY' must be set");
    let aes_iv: String = env::var("AES_IV").expect("'AES_IV' must be set");
    
    // let json_str = fs::read_to_string(path)?;
    // let aes_json: Value = serde_json::from_str(&json_str)?;

    // let aes_key_str = aes_json.get("aes_key")
    //                           .and_then(Value::as_str)
    //                           .ok_or_else(|| anyhow!("aes_key not found"))?;

    // let aes_iv_str = aes_json.get("aes_iv")
    //                          .and_then(Value::as_str)
    //                          .ok_or_else(|| anyhow!("aes_iv not found"))?;
    
    Ok(AesInfosDTO::new(aes_key.as_str(), aes_iv.as_str()))
}


/*
    Function that decrypts the encrypted password of the nosql object
*/
// pub fn decrypt_nosql_pw(enc_pw: &Option<Vec<u8>>, aes_info: &AesInfosDTO) -> Result<String, anyhow::Error> {

//     let parsing_pw = match enc_pw {
        
//         Some(ref pw_enc) => {
            
//             if pw_enc.is_empty() {
//                 String::from("")
//             } else {
                 
//                 let dec_pw = match decrypt(pw_enc, aes_info.aes_key(), aes_info.aes_iv()) {
//                     Ok(dec_pw) => dec_pw,
//                     Err(err) => {
//                         error!("{:?}", err);
//                         return Err(anyhow!("Encrypted data has failed to decrypt."));
//                     }
//                 };

//                 let dec_pw_str = String::from_utf8_lossy(&dec_pw).to_string();
//                 get_url_encoding(dec_pw_str.as_str())
//             }
//         },
//         None => String::from("")
//     };

//     Ok(parsing_pw)
// }