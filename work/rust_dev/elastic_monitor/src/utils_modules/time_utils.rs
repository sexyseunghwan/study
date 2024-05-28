use crate::common::*;

/*
    Function that returns the current time in string format  
*/
pub fn get_current_utc_time(time_format: &str) -> String {
    
    let now: DateTime<Utc> = Utc::now();
    
    return now.format(time_format).to_string();
}


/*
    Function that returns the current time in "NaiveDate" format  
*/
pub fn get_curr_utc_time_naive() -> NaiveDateTime {
    
    let now: DateTime<Utc> = Utc::now();
    
    now.naive_utc()
}


/*
    Function that displays a specific number up to two decimal places.
*/
pub fn round_to_two_decimal_places(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}
