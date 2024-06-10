
use crate::common::*;

//use crate::utils_modules;
use crate::utils_modules::dec_utils::*;
use crate::utils_modules::logger_utils::*;

use crate::service::kafka_service::*;
use crate::service::mysql_async_service::*;
use crate::service::monitoring_service::*;


/*
    ======================================================
    ============= Monitoring Main Controller =============
    ======================================================
*/
pub async fn main_controller() {

    // Record program start time for performance measurement
    let start_time = Instant::now();
    
    // Select compilation environment
    dotenv().ok();
    
    let c_version: String = env::var("COMPILE_ENV").expect("'COMPILE_ENV' must be set");
    let db_url = env::var("DB_URL").expect("'DB_URL' must be set");
    let kafka_host = env::var("KAFKA_HOST").expect("'KAFKA_HOST' must be set");
    
    // Kafka connection -> Producer Connection
    let kafka_client = match ProduceBroker::new(&kafka_host) {
        Ok(kafka_client) => kafka_client,
        Err(err) => {
            error!("Failed to create Kafka client: {:?}", err);
            panic!("Failed to create Kafka client: {:?}", err)
        }
    };
    
    // MySQL connection
    let mysql_client = match MySqlAsyncClient::new(&db_url).await {
        Ok(mysql_client) => mysql_client,
        Err(err) => {
            error!("Failed to create mysql client: {:?}", err);
            panic!("Failed to create mysql client: {:?}", err);
        }
    };
    
    // Get AES infos
    let aes_infos = match get_aes_infos() {
        Ok(aes_infos) => aes_infos,
        Err(err) => {
            error!("Failed to convert AesInfos object: {:?}", err);
            panic!("Failed to convert AesInfos object: {:?}", err);
        }
    };
    
    
    if c_version == "test" || c_version == "debug" || c_version == "local"  {        
        
        match metric_monitor(&kafka_client, &mysql_client, &aes_infos).await {
            Ok(_) => () ,
            Err(err) => {
                error!("{}",err)
            }
        }
        
    } else {
        
        // for prod
        loop {
            
            monitor_thread_stop();
            
            match metric_monitor(&kafka_client, &mysql_client, &aes_infos).await {
                Ok(_) => (),
                Err(err) => {
                    error!("{}",err)
                }
            }
            
            //break;
            thread::sleep(Duration::from_secs(60));
        }
    }
    
    let duration = start_time.elapsed(); 
    println!("Time elapsed: {:?}", duration);
    
}