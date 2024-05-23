use crate::common::*;

use crate::configure::kafka_global_config::*;

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
    Custom Log Format Function
*/
fn common_log_format(log_level: log::Level, msg: &str) -> String {

    // Gets the current time and converts it to UTC DateTime.
    let now = SystemTime::now();
    let datetime: DateTime<Utc> = now.into();
    let timestamp = datetime.format("%Y-%m-%d %H:%M:%S");

    // Gets the name of the currently running thread.
    let thread_name = std::thread::current().name().unwrap_or("unknown").to_string();

    format!("[{}] [{}] T[{}] {}", timestamp, log_level, thread_name, msg)
}


/*
    Error handler used throughout the program
*/
pub async fn errors<T: Debug>(err: &T) {

    let error_str = format!("{:?}", err);

    error!("{:?}", err);
    
    let formatted_msg = common_log_format(log::Level::Error, &error_str);
    
    let _ = match logger_kafka_global("rust-web", &formatted_msg).await {
        Ok(_) => (),
        Err(err) => error!("{:?}", err)
    };
}


/*
    Info handler used throughout the program
*/
pub async fn infos(msg: &str) {
    
    let formatted_msg = common_log_format(log::Level::Info, msg);

    info!("{:?}", msg);
    
    let _ = match logger_kafka_global("rust-web", &formatted_msg).await {
        Ok(_) => (),
        Err(err) => error!("{:?}", err)
    };
    
}