/*
Author      : Seunghwan Shin 
Create date : 2023-02-06 
Description : 
    
History     : 2023-02-06 Seunghwan Shin       # first create => Do not perform DELETE; only logging is performed.
              2023-03-28 Seunghwan Shin       # Change the "MySQL" connection method to asynchronous processing.(SYNC -> ASYNC)
              2023-04-04 Seunghwan Shin       # Change the source to the way it is referenced in the DB information ".env" file for system security
              2024-05-23 Seunghwan Shin       # Add kibana_url information for immediate access using kibanaurl in the event of an elasticsearch cluster problem
              2024-06-07 Seunghwan Shin       # Change source code to cause panic to exit the program if MySQL initial connection fails.
              2024-06-00 Seunghwan Shin       # Change source code to cause panic to exit the program if MySQL initial connection fails.
*/ 
mod common;

mod controller;
use controller::main_controller::*;

mod dto;

mod service;

mod utils_modules;
use utils_modules::logger_utils::*;

#[tokio::main]
async fn main() {
    
    // Initiate Logger
    set_global_logger();
    
    // Start Controller 
    main_controller().await;

}