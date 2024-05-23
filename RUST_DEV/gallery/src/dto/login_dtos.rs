use crate::common::*;

#[derive(Debug, FromRow, Deserialize, Getters, Setters, new)]
#[getset(get = "pub")]
pub struct UserLoginInput {
    user_id: String,
    user_pw: String,
}

// #[derive(Debug, Queryable, FromRow, Deserialize, Getters, Setters, new)]
// #[getset(get = "pub")]
// pub struct UserLoginPreData {
//     user_seq: i32,
//     user_id: String,
//     user_pw: String,
// }

#[derive(Debug, Queryable, FromRow, Serialize, Deserialize, Getters, Setters, new)]
#[getset(get = "pub")]
pub struct UserData {
    user_seq: i32,
    user_id: String,
    user_pw: String,
    user_name: Option<String>
}

#[derive(Debug, Queryable, FromRow, Serialize, Deserialize, Getters, Setters, new)]
#[getset(get = "pub")]
pub struct UserPostData {
    user_seq: i32,
    user_id: String,
    user_name: Option<String>
}

/*
    Jwt token information object
    access_token : "access token" information
    refresh_token : "refresh_token" information
*/
#[derive(Debug, Getters, Setters, new)]
#[getset(get = "pub")]
pub struct JwtTokens {
    pub access_token: String,
    pub refresh_token: String
}

#[derive(Debug, Getters, Setters, new)]
#[getset(get = "pub", set = "pub")]
pub struct ReToken {
    pub token_is_valid: bool,
    pub access_token: Option<String>
}

#[derive(Debug, Serialize, Clone, Deserialize, Getters, new)]
#[getset(get = "pub")]
pub struct Claims {
    pub user_seq: i32,      
    pub company: String,
    pub exp: u64,           
}


