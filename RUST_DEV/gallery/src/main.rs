mod common;
use common::*;

mod middleware;

mod dto;

mod services;

mod controller;
use controller::bind_controller::*;

mod util_mod;
use util_mod::logger_utils::*;

mod repositories;

mod configure;

mod dicontainer;

mod macros;

// ORM Related Crates.
extern crate diesel;

mod schema;


#[actix_web::main]
async fn main() {

    set_global_logger();

    match bind_controller().await {
        Ok(_) => (),
        Err(e) => {
            error!("{:?}", e);
        }
    }
}      