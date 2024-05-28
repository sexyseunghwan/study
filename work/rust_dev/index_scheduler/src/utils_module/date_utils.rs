use crate::common::*;

/*
    Function that returns the current time in "NaiveDate" format  
*/
pub fn get_curr_utc_time() -> Result<NaiveDate, String> {
    
    let now: DateTime<Utc> = Utc::now();
    
    match NaiveDate::from_ymd_opt(now.year(), now.month(), now.day()) {
        Some(date) => Ok(date),
        None => Err("Invalid date.".to_string())
    }
}


/*
    Function that adds or subtracts a specific number of days from a given date.
*/
pub fn get_calculate_time(date: NaiveDate, day_cnt: i64) -> NaiveDate {

    let calcul_date: NaiveDate = date - cDuration::days(day_cnt);

    calcul_date
}
