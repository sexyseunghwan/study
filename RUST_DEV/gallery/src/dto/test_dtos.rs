use crate::common::*;


#[derive(Debug, Deserialize, Getters, Setters, new)]
#[getset(get = "pub")]
pub struct QueryInfo {
    param1: String,
    param2: i64,
}


#[derive(Debug, Deserialize, Getters, Setters, new)]
#[getset(get = "pub")]
pub struct PostInfo {
    sql: String
}