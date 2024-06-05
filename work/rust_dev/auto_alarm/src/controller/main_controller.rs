use crate::common::*;

use crate::service::kafka_service::*;
use crate::service::mysql_async_service::*;
use crate::service::alarm_service::*;


/*
    Controller to act as a mainstream.
*/
pub async fn main_controller() {

    // Record program start time for performance measurement
    let start_time = Instant::now();

    // Select compilation environment
    dotenv().ok();

    //let c_version = env::var("COMPILE_ENV").expect("Compile type must be set");
    let rdb_url = env::var("RDB_URL").expect("'RDB_URL' must be set");
    let kafka_host = env::var("KAFKA_HOST").expect("'KAFKA_HOST' must be set");
    
    // Kafka connection
    let kafka_client = match KafkaBroker::new(&kafka_host) {
        Ok(kafka_client) => kafka_client,
        Err(err) => {
            error!("Failed to create Kafka client: {:?}", err);
            return;
        }
    };
    
    // MySQL connection
    let mysql_client = match MySqlAsyncClient::new(&rdb_url).await {
        Ok(mysql_client) => mysql_client,
        Err(err) => {
            error!("Failed to create mysql client: {:?}", err);
            panic!("Failed to create mysql client: {:?}", err);
        }
    };
    
    /*
        KAFKA -- consuming --> alarm data --> catch --> SEND Telegram Bot
    */
    match push_alarm_to_telebot(kafka_client, mysql_client).await {
        Ok(_) => (),
        Err(err) => {
            error!("{:?}", err);
        } 
    }
    
    let duration = start_time.elapsed(); 
    println!("Time elapsed: {:?}", duration);


}