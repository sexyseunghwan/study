use crate::common::*;


/*

*/
pub fn get_cookie(req: &ServiceRequest, name: &str) -> String {
    
    let http_req = req.request().clone();

    match http_req.cookie(name) {
        Some(user_seq) => {
            user_seq.value().to_string()
        },
        None => String::from("-1")
    }
}


/*
    
*/
pub fn get_cookie_from_http(req: &HttpRequest, name: &str) -> String {

    match req.cookie(name) {
        Some(user_seq) => {
            user_seq.value().to_string()
        },
        None => String::from("-1")
    }
}


/*
    Function that sets cookies.
*/
fn set_cookie(cookie_name: String, cookie_value: String, duration: time::Duration) -> Cookie<'static> {
    
    let expires_at = duration + time::Duration::hours(9);
    
    Cookie::build(cookie_name, cookie_value)
        .path("/")
        .secure(true)       // Send cookies only through HTTPS.
        .http_only(true)    // Prevent access through JavaScript.
        .max_age(expires_at)
        .same_site(SameSite::None)
        .finish() 
}

pub fn set_cookie_per_days(cookie_name: String, cookie_value: String, duration_day: i64) -> Cookie<'static> {
    set_cookie(cookie_name, cookie_value, time::Duration::days(duration_day))
}

pub fn set_cookie_per_hours(cookie_name: String, cookie_value: String, duration_hour: i64) -> Cookie<'static> {
    set_cookie(cookie_name, cookie_value, time::Duration::hours(duration_hour))
}

pub fn set_cookie_per_mins(cookie_name: String, cookie_value: String, duration_min: i64) -> Cookie<'static> {
    set_cookie(cookie_name, cookie_value, time::Duration::minutes(duration_min))
}