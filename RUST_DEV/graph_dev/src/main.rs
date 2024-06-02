mod common;

mod plotters_dev;

mod logger_util;

use logger_util::*;
use plotters_dev::*;
use common::*;

fn main() {
    
    // Initiate Logger
    set_global_logger();

    match draw_test() {
        Ok(_) => (),
        Err(e) => {
            error!("{:?}", e)
        }
    }

}
