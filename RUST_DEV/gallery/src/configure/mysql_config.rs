use crate::common::*;

use crate::util_mod::global_var_utils::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct MySqlInfo {
    ip_address: String,
    port: u16,
    database_name: String,
    username: String,
    password: String
}

/* 
    
*/
pub fn init_rdb_conn() -> Result<mysql_async::Pool, anyhow::Error> {

    let database_url = env::var("DATABASE_URL").expect("'DATABASE_URL' must be set");
    let opts_url = mysql_async::Opts::from_url(&database_url)?;
    let conn_pool: Pool = mysql_async::Pool::new(opts_url);

    Ok(conn_pool)
}


#[derive(Debug)]
pub struct MySqlClient {
    db_pool: mysql_async::Pool
}

impl MySqlClient {
    
    /*
        NEW constructor.
    */
    pub fn new(db_pool: mysql_async::Pool) -> Self {
                
        MySqlClient { db_pool }
    }
    

    /*
        Functions that import the connection from the MySQL Connection pool.
    */
    async fn get_conn_from_pool(&self) -> Result<mysql_async::Conn, anyhow::Error> {
    
        let self_db_pool = &self.db_pool;
        
        self_db_pool.get_conn().await.map_err(|e| anyhow::Error::new(e))

    }
    
    /*
        Dynamic query execution function. [SELECT]
    */
    pub async fn query_select_from_param<T,P>(&self, query: &str, params: P) -> Result<Vec<T>, anyhow::Error> where T: FromRow + std::marker::Send , P:Into<mysql_async::Params> + std::marker::Send {
        
        let mut conn = self.get_conn_from_pool().await?;

        let query_res = conn.exec_map(query, params, T::from_row).await?;

        Ok(query_res)
    }
    
    
    /*
        Static query execution function.[SELECT]
    */
    pub async fn query_select_from<T>(&self, query: &str) -> Result<Vec<T>, anyhow::Error> where T: FromRow + std::marker::Send {

        let mut conn = self.get_conn_from_pool().await?;

        let query_res = conn.query_map(query, T::from_row).await?;

        Ok(query_res)
    }
    
    
    /*
        Dynamic query execution function. [INSERT]
    */
    pub async fn query_insert_from<P>(&self, query: &str, params: P) -> Result<u64, anyhow::Error> where P:Into<mysql_async::Params> + std::marker::Send {

        let mut conn = self.get_conn_from_pool().await?;

        let result = conn.exec_iter(query, params).await?;
        
        match result.last_insert_id() {
            Some(last_insert_id) => Ok(last_insert_id),
            None => {
                Err(anyhow::Error::msg("The insert operation failed."))
            }
        }
    }   

    /*
        Dynamic query execution function. [UPDATE]
    */
    pub async fn query_update_from<P>(&self, query: &str, params: P) -> Result<u64, anyhow::Error> where P:Into<mysql_async::Params> + std::marker::Send {

        let mut conn = self.get_conn_from_pool().await?;

        let result = conn.exec_iter(query, params).await?;

        Ok(result.affected_rows())
    } 


    /*
        Dynamic query execution function. [DELETE]
    */
    pub async fn query_delete_from<P>(&self, query: &str, params: P) -> Result<u64, anyhow::Error> where P:Into<mysql_async::Params> + std::marker::Send {

        let mut conn = self.get_conn_from_pool().await?;

        let result = conn.exec_iter(query, params).await?;
           
        Ok(result.affected_rows())
    } 
    
}