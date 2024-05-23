use crate::common::*;

use crate::configure::mysql_config::*;
use crate::configure::diesel_config::*;
use crate::configure::redis_config::*;

use crate::dto::login_dtos::*;

// ORM OBJECT
use crate::schema::user_tbl_test::dsl::*;


#[async_trait]
pub trait TAuthRepository {
    fn new(db_pools: &web::Data<DbState>) -> Self;//Result<Self, anyhow::Error> where Self: std::marker::Sized;

    // test - classic
    async fn insert_user_info(&self, user_id_str: &str, user_pw_str: &str) -> Result<u64, anyhow::Error>;
    //async fn update_user_info(&self, user_seq: u64, user_id: &str, user_pw: &str) -> Result<u64, anyhow::Error>;
    //async fn delete_user_info(&self, user_seq: u64) -> Result<u64, anyhow::Error>;
    
    async fn find_user_infos_injection(&self, user_id_str: &str, user_pw_str: &str) -> Result<Vec<UserPostData>, anyhow::Error>;

    // test - ORM
    //async fn find_by_id_orm(&self, user_id_str: &str) -> Result<Vec<UserLoginInfo>, anyhow::Error>;
    async fn find_user_seq_by_id_orm(&self, user_id_str: &str) -> Result<Vec<UserData>, anyhow::Error>;
    async fn find_user_infos_orm(&self, user_seq_i32: i32) -> Result<Vec<UserPostData>, anyhow::Error>;

    // use
    //async fn get_user_hashed_pw(&self, user_id: &str) -> Result<String, anyhow::Error>;
    //async fn find_by_id(&self, user_id: &str) -> Result<Vec<UserLoginInfo>, anyhow::Error>;

    async fn set_refresh_token(&self, user_seq_in: i32, refresh_token: &str) -> Result<(), anyhow::Error>;
}

#[derive(Debug)]
pub struct AuthRepository {
    mysql_client: MySqlClient
,   diesel_client: MySqlDieselClient
,   redis_client: RedisHelper
}

#[async_trait]
impl TAuthRepository for AuthRepository {

    fn new(db_pools: &web::Data<DbState>) -> Self {
        
        let mysql_classic_pool = db_pools.mysql_classic_pool.clone();
        let mysql_diesel_pool = db_pools.mysql_diesel_pool.clone();
        
        let mysql_client = MySqlClient::new(mysql_classic_pool);
        let diesel_client = MySqlDieselClient::new(mysql_diesel_pool);

        let redis_conn = db_pools.redis_conn.clone();
        let redis_client = RedisHelper::new(redis_conn);
        
        AuthRepository { mysql_client, diesel_client, redis_client }
    }
    

    //let query = format!("SELECT user_seq, user_id, user_name FROM user_tbl_test WHERE user_id = '{}' AND user_pw = '{}'", user_id_str, user_pw_str);

    /*

    */
    async fn find_user_infos_injection(&self, user_id_str: &str, user_pw_str: &str) -> Result<Vec<UserPostData>, anyhow::Error> {


        let mut conn: diesel::r2d2::PooledConnection<ConnectionManager<MysqlConnection>> = self.diesel_client.get_conn_from_diesel_conn().await?;

        let user_infos = (user_id_str.to_string(), user_pw_str.to_string());

        tokio::task::spawn_blocking(move || {
            
            let user_id_moved = user_infos.0;
            let user_pw_moved = user_infos.1;

            user_tbl_test
                    .filter(user_id.eq(user_id_moved))
                    .filter(user_pw.eq(user_pw_moved))
                    .select((user_seq, user_id, user_name))
                    .load::<UserPostData>(&mut conn)
                    .map_err(anyhow::Error::from)
                    
        })
        .await
        .map_err(|e: task::JoinError| anyhow::Error::new(e))?

    }


    
    // let query = r"
    //     SELECT 
    //         user_seq, 
    //         user_id, 
    //         user_name 
    //     FROM user_tbl_test
    //     WHERE user_id = ?
    //     AND user_pw = ?
    // ";

    // let res: Vec<UserPostData> = self.mysql_client.query_select_from_param(query, (user_id_str, user_pw_str)).await?;


    /*
        test
    */
    async fn insert_user_info(&self, user_id_str: &str, user_pw_str: &str) -> Result<u64, anyhow::Error> {
        
        let insert_res = self.mysql_client.query_insert_from (
            r"
            INSERT INTO 
                USER_TBL_TEST 
            (user_id, user_pw) VALUES (?, ?)",
            (user_id_str, user_pw_str,)
        ).await?;
        
        Ok(insert_res)
    }
    
    // async fn update_user_info(&self, user_seq: u64, user_id: &str, user_pw: &str) -> Result<u64, anyhow::Error> {
        
    //     let update_res = self.mysql_client.query_update_from (
    //         r"
    //         UPDATE 
    //             USER_TBL_TEST 
    //         SET 
    //             user_id = ?, 
    //             user_pw = ?
    //         WHERE 
    //             user_seq = ?",
    //         (user_id, user_pw, user_seq,)
    //     ).await?;

    //     Ok(update_res)
    // }

    // async fn delete_user_info(&self, user_seq: u64) -> Result<u64, anyhow::Error> {
        
    //     let delete_res = self.mysql_client.query_delete_from (
    //         r"
    //         DELETE FROM 
    //             USER_TBL_TEST 
    //         WHERE 
    //             user_seq = ?",
    //         (user_seq, )
    //     ).await?;
        
    //     Ok(delete_res)
    // }

    // /*

    // */
    // async fn find_by_id(&self, user_id: &str) -> Result<Vec<UserLoginInfo>, anyhow::Error> {
        
    //     let test: Vec<UserLoginInfo> = self.mysql_client.query_select_from_param (
    //         r"
    //         SELECT 
    //             user_id, 
    //             user_pw 
    //         FROM 
    //             USER_TBL_TEST 
    //         WHERE 
    //             user_id = ?", 
    //         (user_id,)).await?;

    //     Ok(test)
    //     // match self.mysql_client.query_select_from_param(
    //     //     r"SELECT user_id, user_pw FROM USER_TBL_TEST WHERE user_id = ?", 
    //     //     (user_id,)).await {
    //     //         Ok(user_info) => { 
    //     //             info!("{:?}", user_info);
    //     //             user_info.into_iter().next() 
    //     //         },
    //     //         Err(err) => {
    //     //             error!("{:?}", err);
    //     //             None
    //     //         }
    //     //     }
    // }

    
    /*
        Result<Vec<UserLoginInfo>, anyhow::Error>
    */
    async fn find_user_seq_by_id_orm(&self, user_id_str: &str) -> Result<Vec<UserData>, anyhow::Error> {

        let mut conn: diesel::r2d2::PooledConnection<ConnectionManager<MysqlConnection>> = self.diesel_client.get_conn_from_diesel_conn().await?;
        
        let user_id_move = user_id_str.to_string();
        
        tokio::task::spawn_blocking(move || {
            
            user_tbl_test.filter(user_id.eq(user_id_move))
                    .load::<UserData>(&mut conn)
                    .map_err(anyhow::Error::from)
                    
        })
        .await
        .map_err(|e: task::JoinError| anyhow::Error::new(e))?

    }

    /*
        
    */
    async fn find_user_infos_orm(&self, user_seq_i32: i32) -> Result<Vec<UserPostData>, anyhow::Error> {

        let mut conn: diesel::r2d2::PooledConnection<ConnectionManager<MysqlConnection>> = self.diesel_client.get_conn_from_diesel_conn().await?;

        tokio::task::spawn_blocking(move || {
            
            user_tbl_test.filter(user_seq.eq(user_seq_i32))
                    .select((user_seq, user_id, user_name))
                    .load::<UserPostData>(&mut conn)
                    .map_err(anyhow::Error::from)
                    
        })
        .await
        .map_err(|e: task::JoinError| anyhow::Error::new(e))?
    }

    /*
        , user_seq_in: &str, refresh_token: &str
    */
    async fn set_refresh_token(&self, user_seq_in: i32, refresh_token: &str) -> Result<(), anyhow::Error> {
        
        let mut conn = self.redis_client.get_redis_conn().await?;

        let redis_key = format!("user::{}",user_seq_in);
        
        conn.set(redis_key, refresh_token).await?;

        Ok(())
    }

}