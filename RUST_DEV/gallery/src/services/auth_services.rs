use crate::common::*;

use crate::dto::login_dtos::*;

use crate::repositories::auth_repository::*;

use crate::configure::redis_global_config::*;

use crate::util_mod::bcrypt_utils::*;
use crate::util_mod::jwt_utils::*;
use crate::util_mod::cookies_utils::*;


#[async_trait]

pub trait TAuthService {
    
    async fn verify_user_info(&self, user_id: &str, user_pw: &str) -> Result<i32, anyhow::Error>;
    async fn set_user_jwt_token(&self, user_seq: i32) -> Result<(Cookie<'static>, Cookie<'static>, Cookie<'static>), anyhow::Error>;
    async fn get_user_infos(&self, user_seq: &str) -> Result<UserPostData, anyhow::Error>;
    //async fn verify_user_token(&self, user_jwt: &str) -> Result<Claims, anyhow::Error>;
    
    async fn login_user_injection(&self, user_id: &str, user_pw: &str) -> Result<UserPostData, anyhow::Error>;

    async fn join_membership(&self, user_id: &str, user_pw: &str) -> Result<(), anyhow::Error>;
    async fn verify_user_password(&self, user_pw: &str) -> Result<(), anyhow::Error>;
}

#[derive(Debug)]
pub struct AuthService<T: TAuthRepository + 'static> {
    pub repository: Arc<T>,
}

#[async_trait]
impl<T: TAuthRepository + Sync + Send> TAuthService for AuthService<T> { 
    

    /*

    */
    async fn login_user_injection(&self, user_id: &str, user_pw: &str) -> Result<UserPostData, anyhow::Error> {
        
        let user_info_vec: Vec<UserPostData> = self.repository.find_user_infos_injection(user_id, user_pw).await?;

        let user_data = match user_info_vec.into_iter().next() {
            Some(user_data) => user_data,
            None => { return Err(anyhow::Error::msg("ID or password does not match.")) } 
        };

        Ok(user_data)

    }

    /*
        Function that verifies that the member information handed over by the client MATCHES the stored information.
    */
    async fn verify_user_info(&self, user_id: &str, user_pw: &str) -> Result<i32, anyhow::Error> {
        
        // 1. Gets the user's information filtered by user_id.
        let user_data_vec = self.repository.find_user_seq_by_id_orm(user_id).await?;
        let user_data = match user_data_vec.iter().next() {
            Some(user_data) => user_data,
            None => { return Err(anyhow::Error::msg("ID or password does not match.")) } 
        };
        
        // 2. Compare the hashed password of the user with the password entered by the client.
        let verify_pw = verify_password(user_pw, user_data.user_pw()).await?;
        if verify_pw == false { return Err(anyhow::Error::msg("ID or password does not match.")) }

        Ok(*user_data.user_seq())
    }

    /*
        Function to issue JWT to client
        - The access_token and the refresh_token are generated again, and the refresh_token is stored in Redis.
    */
    async fn set_user_jwt_token(&self, user_seq: i32) -> Result<(Cookie<'static>, Cookie<'static>, Cookie<'static>), anyhow::Error> {
        
        // Prod
        //let new_access_token = generate_token(user_seq, true, 604800).await?; // 7 day
        //let new_refresh_token = generate_token(user_seq, false, 2592000).await?; // 30 day

        // Test
        let new_access_token = generate_token(user_seq, true, 2).await?; // 2min
        let new_refresh_token = generate_token(user_seq, false, 10).await?; // 10min
        
        // The "refresh token" is stored in Redis.
        let mut redis_conn: redis::cluster_async::ClusterConnection = get_global_redis_conn().await?;
        let redis_refresh_token_key = format!("jwt_user::{}", user_seq);
        redis_conn.set(redis_refresh_token_key, &new_refresh_token).await?;


        let access_token_cookie = set_cookie_per_days(String::from("access_token"), new_access_token, 2);
        let refresh_token_cookie = set_cookie_per_days(String::from("refresh_token"), new_refresh_token, 10);
        let user_seq_cookie = set_cookie_per_days(String::from("user_seq_num"), user_seq.to_string(), 10);
        
        Ok((access_token_cookie, refresh_token_cookie, user_seq_cookie))
    }

    /*
        
        1) Length: At least 8 characters.
        2) Content: Contains only alphanumeric characters or special characters.
        3) Uppercase: Includes at least one uppercase letter.
        4) Special Character: Includes at least one special character.
        5) Sequential Numbers: Does not contain sequential numbers.
    */
    async fn verify_user_password(&self, user_pw: &str) -> Result<(), anyhow::Error> {

        let user_pw_move = user_pw.to_string();

        task::spawn_blocking(move|| {
            
            let user_pw_moved = user_pw_move;

            let length_check = user_pw_moved.as_str().len() >= 8;
            let content_check = Regex::new(r"^[A-Za-z0-9!@#$%^&*()_+=-]{8,}$")?.is_match(user_pw_moved.as_str());
            let uppercase_check = Regex::new(r"[A-Z]")?.is_match(user_pw_moved.as_str());
            let special_char_check = Regex::new(r"[!@#$%^&*()_+=-]")?.is_match(user_pw_moved.as_str());
            let sequential_numbers_check = !Regex::new(r"(012|123|234|345|456|567|678|789)")?.is_match(user_pw_moved.as_str());

            if !length_check { return Err(anyhow!("The password must be at least 8 digits long."))}
            if !content_check { return Err(anyhow!("The passwords can only be in English, numbers, or special characters."))}
            if !uppercase_check { return Err(anyhow!("The password must contain at least one English uppercase letter."))}
            if !special_char_check { return Err(anyhow!("The password must contain at least one special character."))}
            if !sequential_numbers_check { return Err(anyhow!("The passwords cannot be consecutive numbers."))}

            Ok(())

        }).await?
   
    }
    
    
    /*
        
    */
    async fn join_membership(&self, user_id: &str, user_pw: &str) -> Result<(), anyhow::Error> {

        // Check password violation policy
        let _ = self.verify_user_password(user_pw).await?;
        
        // Apply the hash algorithm to the password.
        let hashed_pw = encryption_by_hash(user_pw).await?;
        
        // Insert New USER DATA
        let _ = match self.repository.insert_user_info(user_id, &hashed_pw).await {
            Ok(res) => {
                let sign_msg = format!("user_id : {}, user_seq : {} successfully sign up", user_id, res);
                infos(&sign_msg).await;
            },
            Err(e) => {
                return Err(anyhow!("{:?}", e));
            }
        };
        
        Ok(())
    }
    
    /*

    */
    async fn get_user_infos(&self, user_seq: &str) -> Result<UserPostData, anyhow::Error> {
        
        let user_seq_num = user_seq.parse()?;

        let user_info_vec: Vec<UserPostData> = self.repository.find_user_infos_orm(user_seq_num).await?;

        let user_data = match user_info_vec.into_iter().next() {
            Some(user_data) => user_data,
            None => { return Err(anyhow::Error::msg("ID or password does not match.")) } 
        };

        Ok(user_data)
    }
}   