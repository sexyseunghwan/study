use crate::common::*;
use crate::data_obj::*;

/*
    Function responsible for logging
*/
pub fn set_global_logger() {
    
    let current_date = Local::now().format("%Y_%m_%d").to_string();
    let file_suffix = format!("{}.log", current_date);
    
    //logger settings
    flexi_logger::Logger::with_str("info")
        .log_to_file()
        .directory("logs") // Specify directory to store logs
        .rotate(
            flexi_logger::Criterion::Age(flexi_logger::Age::Day), // every day
            flexi_logger::Naming::Timestamps, // With date-based names
            flexi_logger::Cleanup::KeepLogFiles(10)
        )
        .format(flexi_logger::colored_with_thread)
        .suffix(file_suffix)
        .start()
        .unwrap();
}


/*
    Function that returns the current time in "NaiveDate" format  
*/
pub fn get_curr_utc_time() -> Result<NaiveDate, String> {
    
    let now: DateTime<Utc> = Utc::now();
    
    match NaiveDate::from_ymd_opt(now.year(), now.month(), now.day()) {
        Some(date) => Ok(date),
        None => Err("Invalid date.".to_string())
    }
}


/*
    Function that adds or subtracts a specific number of days from a given date.
*/
pub fn get_calculate_time(date: NaiveDate, day_cnt: i64) -> NaiveDate {

    let calcul_date: NaiveDate = date - cDuration::days(day_cnt);

    calcul_date
}


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
pub fn get_aes_infos(path: &str) -> Result<AesInfos, anyhow::Error> {
    
    let json_str = fs::read_to_string(path)?;
    let aes_json: Value = serde_json::from_str(&json_str)?;

    let aes_key_str = aes_json.get("aes_key")
                              .and_then(Value::as_str)
                              .ok_or_else(|| anyhow!("aes_key not found"))?;

    let aes_iv_str = aes_json.get("aes_iv")
                             .and_then(Value::as_str)
                             .ok_or_else(|| anyhow!("aes_iv not found"))?;
    
    Ok(AesInfos::new(aes_key_str, aes_iv_str))
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


/*
    Function that CONVERTS Elasticsearch cluster JSON information for monitoring into OBJECTS
*/
// pub fn get_parsing_es_info_version(c_version: &str) -> NoSqlClusterInfo {
    
//     let es_file_path = if c_version == "test" {
//         "./data_file/mon_es_info/es_info_dev.json"
//     } else if c_version == "debug" {
//         "./data_file/mon_es_info/es_info_prod.json"
//     } else {
//         "./data_file/mon_es_info/es_info_prod.json"
//     };
    
//     let parse_es_data = fs::read_to_string(&es_file_path).expect("An error occurred while opening the ES information file."); 
    
//     let file_es_info: NoSqlClusterInfo = serde_json::from_str(&parse_es_data).expect("An error occurred while parsing the ES information file json information.");
    
//     return file_es_info
// }


// TESTING

// fn err_test() -> Result<(), Box<dyn Error>> {

//     return Err(Box::<dyn Error>::from("public void main"));
// }

// pub fn throw_test() -> Result<(), Box<dyn Error>> {

//     err_test()?;

//     println!("??");

//     Ok(())
// }

//test
// pub fn window() -> Result<(), Box<dyn Error>> {

//     let test = String::from("nd-partner-log.2024.01.09\nnd-partner-log.2024.01.10\nnd-partner-log.2024.01.11\n");

//     let res_test: Vec<&str> = test.split("\n").filter(|&d| !d.is_empty()).collect();

//     println!("{:?}",res_test);

//     Ok(())
// }