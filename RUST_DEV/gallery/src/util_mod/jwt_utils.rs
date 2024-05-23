use crate::common::*;

use crate::dto::login_dtos::*;

use crate::configure::redis_global_config::*;


/*
    Function that globally initializes a SECRET KEY that will perform verification of access and refresh tokens.
*/
pub async fn init_jwt_private_key() -> Result<(), anyhow::Error> {

    let access_key = env::var("ACCESS_KEY").expect("'ACCESS_KEY' must be set");
    let refresh_key = env::var("REFRESH_KEY").expect("'REFRESH_KEY' must be set");
    
    let mut access_guard = ACCESS_KEY.write().await;
    *access_guard = access_key;

    let mut refresh_guard = REFRESH_KEY.write().await;
    *refresh_guard = refresh_key;

    Ok(())

}


/*
    Function that issues JWT.
*/
pub async fn generate_token(user_seq: i32, access_token_yn: bool, expire_min: u64) -> Result<String, anyhow::Error> {   

    let private_key_str = 
        if access_token_yn {
            let access_key = ACCESS_KEY.read().await;
            access_key.clone()
        } else {
            let refresh_key = REFRESH_KEY.read().await;
            refresh_key.clone()
        };
    
    let private_key_byte = private_key_str.as_bytes();
    
    let expire_sec = expire_min * 60;

    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs() + expire_sec; 
    
    let claims_clone = Claims::new(user_seq, String::from("seunghwan"), expiration);

    let token = encode(&Header::default(), &claims_clone, &EncodingKey::from_secret(private_key_byte))?;
        
    Ok(token)
}



/*
    Verify the JWT
*/
pub async fn verify_jwt<T: DeserializeOwned>(token: &str, user_seq_num: i32, access_yn: bool) -> Result<T, anyhow::Error> {
    
    if access_yn == false {
        
        let mut redis_conn: redis::cluster_async::ClusterConnection = get_global_redis_conn().await?;
        let redis_find_key = format!("jwt_user::{}", user_seq_num);
        
        let user_refresh_key: String = match redis_conn.get(redis_find_key).await {
            Ok(user_refresh_key) => user_refresh_key,
            Err(e) => return Err(anyhow!("'user_refresh_key' does not exist. : {:?}", e))
        };

        if user_refresh_key != token.to_string() {
            return Err(anyhow!("The refresh key stored in Redis does not match the refresh key stored in the cookie."))
        }
    }

    let private_key_str = 
        if access_yn {
            let access_key = ACCESS_KEY.read().await;
            access_key.clone()
        } else {
            let refresh_key = REFRESH_KEY.read().await;
            refresh_key.clone()
        };
    
    let private_key = private_key_str.as_bytes();
    
    let private_decode_key = DecodingKey::from_secret(private_key.as_ref());
    let validation = Validation::new(Algorithm::HS256);

    decode::<T>(token, &private_decode_key, &validation)
        .map_err(|e| anyhow::Error::new(e))
        .map(|data| data.claims)
    
} 


/*
    Function that verifies the client's jwt information.
*/
pub async fn check_user_jwt_infos(access_token: &str, refresh_token: &str, user_seq_num: i32) -> Result<ReToken, Error> {
    
    let token_object = match verify_jwt::<Claims>(access_token, user_seq_num, true).await {
        Ok(_) => {
            // access_token is valid
            ReToken::new(true, None)
        },
        Err(e) => {
            // access_token is not valid
            error!("{:?}", e);
            
            // refresh_token is valid
            let token_object = match verify_jwt::<Claims>(refresh_token, user_seq_num, false).await {
                Ok(refresh_token) => {
                    
                    let user_seq = *refresh_token.user_seq();
                    
                    // Reissue access_token
                    let re_access_token = 
                        generate_token(user_seq, true, 120).await
                        .map_err(|e| ErrorUnauthorized(format!("Failed to create access token. : {:?}",e)))?;

                    ReToken::new(false, Some(re_access_token))
                },
                Err(e) => {
                    return Err(ErrorUnauthorized(format!("Failed to verify refresh token: {:?}", e)))
                }
            };
            
            token_object
        }
    };

    Ok(token_object)    
}