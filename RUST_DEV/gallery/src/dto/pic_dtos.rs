use crate::common::*;


#[derive(Debug, FromRow, Deserialize, Getters, Setters, new)]
#[getset(get = "pub")]
pub struct BestSeenInput {
    user_seq: i64,
    pic_seq: i64
}