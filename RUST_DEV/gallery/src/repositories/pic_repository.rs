use crate::common::*;

use crate::configure::mysql_config::*;
use crate::configure::diesel_config::*;
use crate::configure::redis_config::*;

// ORM OBJECT
use crate::schema::user_tbl_test::dsl::*;


#[async_trait]
pub trait TPicRepository { 
    fn new(db_pools: &web::Data<DbState>) -> Self;
    
    async fn get_best_seen_pic(&self, pic_cnt: i32) -> Result<(), anyhow::Error>;


    //test
    async fn set_zadd_pic(&self, key_str: &str, user_seq_int: i64, pic_seq: i64) -> Result<(), anyhow::Error>;
    async fn set_zincrby_pic(&self, key_str: &str, pic_seq: i64) -> Result<(), anyhow::Error>;
    async fn get_zscore_pic(&self, key_str: &str, pic_seq: i64) -> Result<i32, anyhow::Error>;
    // async fn set_seen_pic(&self, user_seq_int: i64, pic_seq: i64) -> Result<(), anyhow::Error>;
    // async fn set_seen_pic_count(&self, pic_seq: i64) -> Result<(), anyhow::Error>;
    // async fn get_seen_pic(&self, user_seq_int: i64, pic_seq: i64) -> Result<(), anyhow::Error>;
    // async fn get_seen_pic_count(&self, pic_seq: i64) -> Result<(), anyhow::Error>;
    // async fn get_seen_count_as_rank(&self) -> Result<(), anyhow::Error>;
    //async fn get_seen_pic_check(self, user_seq_int: i64, pic_seq: i64) -> Result<(), anyhow::Error>;
    
}


#[derive(Debug)]
pub struct PicRepository {
    mysql_client: MySqlClient
,   diesel_client: MySqlDieselClient
,   redis_client: RedisHelper
}


#[async_trait]
impl TPicRepository for PicRepository {

    fn new(db_pools: &web::Data<DbState>) -> Self {
        
        let mysql_classic_pool = db_pools.mysql_classic_pool.clone();
        let mysql_diesel_pool = db_pools.mysql_diesel_pool.clone();
        
        let mysql_client = MySqlClient::new(mysql_classic_pool);
        let diesel_client = MySqlDieselClient::new(mysql_diesel_pool);

        let redis_conn = db_pools.redis_conn.clone();
        let redis_client = RedisHelper::new(redis_conn);
        
        PicRepository { mysql_client, diesel_client, redis_client }
    }   


    /*

    */
    async fn get_best_seen_pic(&self, pic_cnt: i32) -> Result<(), anyhow::Error> {

        let redis_conn = self.redis_client.get_redis_conn().await?;

        Ok(())
    }

    
    /*
        zadd key score member
        - key : seenLikes
        - score : timestamp
        - member : userSeq
    */
    async fn set_zadd_pic(&self, key_str: &str, user_seq_int: i64, pic_seq: i64) -> Result<(), anyhow::Error> {
        
        let mut redis_conn: redis::cluster_async::ClusterConnection = self.redis_client.get_redis_conn().await?;
        
        let now_timestamp = Utc::now().timestamp();
        
        let key_str = format!("{}:{}", key_str, pic_seq);
        
        let _ = redis_conn.zadd(key_str, user_seq_int, now_timestamp).await?;
        
        Ok(())
    }
    
    /*
        zincrby key increment member => return : Increased or decreased count value
            - key : count values of specific picture 
            - increment : value
            - member : eigenvalue of picture
    */
    async fn set_zincrby_pic(&self, key_str: &str, pic_seq: i64) -> Result<(), anyhow::Error> {

        let mut redis_conn: redis::cluster_async::ClusterConnection = self.redis_client.get_redis_conn().await?;

        let _: () = redis::cmd("ZINCRBY")
            .arg(key_str)
            .arg(1)
            .arg(pic_seq)
            .query_async(&mut redis_conn)
            .await?;

        Ok(())
    }
    
    
    /*
        zscore key member => return : score(timestamp)
        - key : information of specific photo
        - member : eigenvalue of user
    */
    async fn get_zscore_pic(&self, key_str: &str, pic_seq: i64) -> Result<i32, anyhow::Error> {

        let mut redis_conn: redis::cluster_async::ClusterConnection = self.redis_client.get_redis_conn().await?;

        let res: Option<i32> = redis_conn.zscore(key_str, pic_seq).await?;
        
        match res {
            Some(res) => Ok(res),
            None => Ok(-1)
        }
    }
    
    /*
        zscore key member => return : score(timestamp)
        - key : view information of specific photo
        - member : eigenvalue of user
    */
    // async fn get_seen_pic(&self, user_seq_int: i64, pic_seq: i64) -> Result<(), anyhow::Error> {

    //     let mut redis_conn: redis::cluster_async::ClusterConnection = self.redis_client.get_redis_conn().await?;

    //     let key_str = format!("seenPhoto:{}", pic_seq);

    //     let res: i32 = redis_conn.zscore(key_str, user_seq_int).await?;

    //     Ok(())
    // }
    



    
    /*

    */
    // async fn get_seen_count_as_rank(&self) -> Result<(), anyhow::Error> {
        
    //     let mut redis_conn: redis::cluster_async::ClusterConnection = self.redis_client.get_redis_conn().await?;

    //     let res: Vec<String> = redis_conn.zrevrange("photoSeenCounts", 0, 10).await?;
        
    //     println!("{:?}", res);

    //     Ok(())
    // }

    /*

    */
    // async fn get_seen_pic_check(self, user_seq_int: i64, pic_seq: i64) -> Option<i64> {

    //     let mut redis_conn: redis::cluster_async::ClusterConnection = self.redis_client.get_redis_conn().await?;
        
    //     let key_str = format!("seenLikes:{}", pic_seq);

    //     //let score: Option<i64> = redis_conn.zscore(key_str, user_seq_int).await?;



    //     //Ok(())
    // }


}  