/*
Author      : Seunghwan Shin 
Create date : 2023-02-06 
Description : 
    
History     : 2023-02-06 Seunghwan Shin       # first create
              2024-03-04 Seunghwan Shin       # adding metrics_mapping element
              2024-05-23 Seunghwan Shin       # Add kibana_url information for immediate access using kibanaurl in the event of an elasticsearch cluster problem
              2024-06-00 Seunghwan Shin       # 
*/ 
mod common;
mod controller;
mod utils_modules;
mod service;
mod dtos;

use utils_modules::logger_utils::*;
use controller::main_controller::*;

#[tokio::main]
async fn main() {
    
    // Initiate Logger
    set_global_logger();

    // Start Controller
    main_controller().await;
}
