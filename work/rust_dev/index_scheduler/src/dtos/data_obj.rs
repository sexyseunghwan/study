use crate::common::*;

use crate::utils_module::crypt_utils::*;
//use crate::utils_module::


/* 
    =========================================================================================
    =========== AES encryption-related key and initialization vector information. ===========
    =========================================================================================
*/
#[derive(Deserialize, Debug, Getters, Setters, MutGetters, Default)]
#[getset(get = "pub")]
pub struct AesInfos {
    aes_key: Vec<u8>,
    aes_iv: Vec<u8>
}

impl AesInfos {
    
    pub fn new(aes_key_str: &str, aes_iv_str: &str) -> Self {
        AesInfos {
            aes_key: aes_key_str.as_bytes().to_vec(),
            aes_iv: aes_iv_str.as_bytes().to_vec(),
        }
    }
}


/* 
    ========================================================
    =========== Index Pattern Information Object ===========
    ========================================================
*/
#[derive(Deserialize, Debug, Getters, Setters, MutGetters, Default, FromRow, new)]
#[getset(get = "pub")]
pub struct IndexPatternInfo {
    index_pattern: String,
    presv_period: i64
}

/* 
    =====================================================
    =========== MySQL table schema structure. ===========
    =====================================================
*/

#[derive(Debug, FromRow, Serialize, Deserialize, Getters, Setters, Default, new)]
#[getset(get = "pub", set = "pub")]
pub struct NosqlScheduleInfo {
    cluster_name: String,
    system_type: String,
    kibana_url: String,
    user_id: Option<String>,
    user_pw_enc: Option<Vec<u8>>,
    system_version: String,
    ssl_option: bool
}


#[derive(Debug, Clone, Serialize, Deserialize, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct NosqlScheduleInfoExtend {
    cluster_name: String,
    system_type: String,
    kibana_url: String,
    user_id: String,
    user_pw: String,
    system_version: String,
    ssl_option: bool,
    host_port_list: Vec<String>
}

impl NosqlScheduleInfoExtend {

    pub fn new(nosql_schedule_info: &NosqlScheduleInfo, host_port_list: Vec<String>, aes_infos: &AesInfos) -> Result<Self, anyhow::Error> {

        // ES user_id
        let parsing_id = match nosql_schedule_info.user_id {
            Some(ref user_id) => user_id.to_string(),
            None => String::from("")
        };
        
        // ES user_pw -> decrypt
        let parsing_pw = match nosql_schedule_info.user_pw_enc {
            Some(ref user_pw_enc) => {
                
                if user_pw_enc.is_empty() {
                    String::from("")
                } else {
                    
                    let dec_pw = match decrypt(user_pw_enc, &aes_infos.aes_key, &aes_infos.aes_iv) {
                        Ok(dec_pw) => dec_pw,
                        Err(err) => {
                            return Err(anyhow!("Encrypted data has failed to decrypt. : {}",err));
                        }
                    };

                    let dec_pw_str = String::from_utf8_lossy(&dec_pw).to_string();
                    get_url_encoding(dec_pw_str.as_str())
                }
            },
            None => String::from("")
        };
        
        let extend_nosql_info = NosqlScheduleInfoExtend {
            cluster_name: nosql_schedule_info.cluster_name().to_string(),
            system_type: nosql_schedule_info.system_type().to_string(),
            kibana_url: nosql_schedule_info.kibana_url().to_string(),
            user_id: parsing_id,
            user_pw: parsing_pw,
            system_version: nosql_schedule_info.system_version().to_string(),
            ssl_option: *nosql_schedule_info.ssl_option(),
            host_port_list
        };
        
        Ok(extend_nosql_info)
    }


}




/*
    ====================================================================== 
    =========== Alarm message-related structure template. ===========
    ======================================================================   
*/

pub trait MsgDetail: Serialize {}

#[derive(Serialize, Deserialize, Debug, Getters, Setters, new)]
#[getset(get = "pub")]
pub struct AlarmDetailInfo {
    pub host_info: String,
    pub metric_type: String,
    pub metric_val: f64,
}

impl MsgDetail for AlarmDetailInfo {}


#[derive(Serialize, Deserialize, Debug, Getters, Setters, new)]
#[getset(get = "pub")]
pub struct LogDetail {
    pub index_name: String,
    pub success_yn: bool,
    pub detail_msg: String
}

impl MsgDetail for LogDetail {}

#[derive(Serialize, Deserialize, Debug, Getters, Setters, new)]
#[getset(get = "pub")]
pub struct AlarmDetailError {
    pub err_content: String,
}

impl MsgDetail for AlarmDetailError {}

#[derive(Serialize, Deserialize, Debug, new)]
pub struct AlarmMetricForm<T> where T: MsgDetail{
    pub alarm_type: String,
    pub monitor_type: String,
    pub cluster_name: String,
    pub kibana_url: String,
    pub contents: Vec<T>
}