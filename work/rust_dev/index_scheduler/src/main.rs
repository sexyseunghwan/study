/*
Author      : Seunghwan Shin 
Create date : 2024-01-16 
Description : Source code that has a function to periodically delete indexes belonging to a specific index pattern in Elasticsearch.
    
History     : 2024-01-16 Seunghwan Shin       # first create => Do not perform DELETE; only logging is performed.
              2024-05-28 Seunghwan Shin       # 1) Change source code to reference .env file
                                                2) Change source code to record all information about success/failure after index deletion
 
*/ 
mod common;
use common::*;

mod controller;
use controller::main_controller::*;

mod service;

mod utils_module;
use utils_module::logger_utils::*;

mod dtos;

#[tokio::main]
async fn main() {
    
    // Initiate Logger
    set_global_logger();
    
    // Start Controller 
    main_controller().await;
    
    // End process.
    info!("The index scheduling process ended normally.");
}
