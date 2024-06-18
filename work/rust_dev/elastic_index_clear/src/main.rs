/*
Author      : Seunghwan Shin
Create date : 2023-06-21
Description : The code is a batch code that removes the largest index.
              Elasticsearch cluster(DEV,QA,STG) disk full is frequently occurring.
              This code operates when the disk occupancy rate is more than 80%.

History     : 2023-06-21 Seunghwan Shin       # first create
              2023-07-17 Seunghwan Shin       # Modified the code to leave a separate "INDEX DELETE" log.
              2023-07-25 Seunghwan Shin       # Create a server list folder and select a cluster to monitor.
                                                Change the source code so that it can be controlled in one place.
              2023-07-26 Seunghwan Shin       # An issue occurred where log files were not created by date.
                                                Changed so that log files can be created by date
              2024-06-18 Seunghwan Shin       # Converting Python Code to RUST Code
*/ 
mod common;
mod controller;
mod utils_modules;
mod service;

use utils_modules::logger_utils::*;
use controller::main_controller::*;

#[tokio::main]
async fn main() {
    
    // Initiate Logger
    set_global_logger();

    // Start Controller
    main_controller().await;
}
