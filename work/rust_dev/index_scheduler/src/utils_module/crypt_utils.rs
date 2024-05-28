use crate::common::*;

use crate::dtos::data_obj::*;

/*
    Function that decrypts an encrypted string. (AES-356)    
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
pub fn get_aes_infos(aes_key: &str, aes_iv: &str) -> Result<AesInfos, anyhow::Error> {
    
    Ok(AesInfos::new(aes_key, aes_iv))
}


/*
    Function that URL-encodes the target string
*/
pub fn get_url_encoding(input_str: &str) -> String {

    const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`')
                                    .add(b'#').add(b'?').add(b'{').add(b'}')
                                    .add(b'/').add(b':').add(b';').add(b'=')
                                    .add(b'@').add(b'[').add(b']').add(b'\\')
                                    .add(b'^').add(b'|');

    utf8_percent_encode(input_str, FRAGMENT).to_string()

}