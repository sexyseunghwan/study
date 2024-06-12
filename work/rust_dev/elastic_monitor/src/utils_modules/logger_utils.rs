use crate::common::*;

use crate::dto::alarm_related_dtos::*;

use crate::service::kafka_service::*;
/*
    Function responsible for logging
*/
pub fn set_global_logger() {
    let log_directory = "logs"; // Directory to store log files
    let file_prefix = ""; // Prefixes for log files

    // Logger setting
    Logger::try_with_str("info")
        .unwrap()
        .log_to_file(FileSpec::default().directory(log_directory).discriminant(file_prefix))
        .rotate(
            Criterion::Age(Age::Day), // daily rotation
            Naming::Timestamps, // Use timestamps for file names
            Cleanup::KeepLogFiles(10) // Maintain up to 10 log files
        )
        .format_for_files(custom_format)
        .start()
        .unwrap_or_else(|e| panic!("Logger initialization failed: {}", e));
}

// Custom Log Format Function
fn custom_format(w: &mut dyn Write, now: &mut flexi_logger::DeferredNow, record: &Record) -> Result<(), std::io::Error> {
    write!(w, "[{}] [{}] T[{}] {}",
        now.now().format("%Y-%m-%d %H:%M:%S"),
        record.level(),
        std::thread::current().name().unwrap_or("unknown"),
        &record.args())
}

/*
    Function that records info logs to a file and also records it to Kafka
*/
pub async fn infos(info_msg: &str) {
    info!("{:?}", info_msg);
    
    let producer_clone = {
        let producer_lock = LOGGER_PRODUCER.lock().expect("Failed to lock producer");
        producer_lock.clone()
    };

    // let producer_lock = match LOGGER_PRODUCER.lock() {
    //     Ok(producer_lock) => producer_lock,
    //     Err(e) => {
    //         error!("{:?}", e);
    //         panic!("Cannot recover from locking failure");
    //     }
    // };

    // let producer_clone = producer_lock.clone();
    // drop(producer_lock); // Explicitly drop the lock

    let msg_detail = LogDetail::new(String::from("ELASTIC_MONITOR"), String::from("INFO"), info_msg.to_string());

    match producer_clone.send_message_to_kafka_log(&msg_detail, "nosql_mon_log").await {
        Ok(_) => (),
        Err(e) => error!("{:?}", e)
    }
}


/*
    Function that records info logs to a file and also records it to Kafka
*/
pub async fn errors(err_msg: anyhow::Error) {
    error!("{:?}", err_msg);

    let producer_clone = {
        let producer_lock = LOGGER_PRODUCER.lock().expect("Failed to lock producer");
        producer_lock.clone()
    };

    // let producer_lock = match LOGGER_PRODUCER.lock() {
    //     Ok(producer_lock) => producer_lock,
    //     Err(e) => {
    //         error!("{:?}", e);
    //         panic!("Cannot recover from locking failure");
    //     }
    // };
    
    // let producer_clone = producer_lock.clone();
    // drop(producer_lock); // Explicitly drop the lock

    let msg_detail = LogDetail::new(String::from("ELASTIC_MONITOR"), String::from("ERROR"), err_msg.to_string());

    match producer_clone.send_message_to_kafka_log(&msg_detail, "nosql_mon_log").await {
        Ok(_) => (),
        Err(e) => error!("{:?}", e)
    }

}