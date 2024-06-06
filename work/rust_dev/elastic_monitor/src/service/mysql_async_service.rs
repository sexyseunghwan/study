use crate::common::*;


#[derive(Debug)]
pub struct MySqlAsyncClient {
    pool: Option<Pool>
}

#[derive(Serialize, Deserialize, Debug)]
struct MySqlAsyncInfo {
    ip_address: String,
    port: u16,
    username: String,
    password: String,
    database_name: String
}

impl MySqlAsyncClient {

    /*
        Constructor of MySqlClient
    */
    pub async fn new(db_url: &str) -> Result<Self, anyhow::Error> {
        
        let mut mysql_client = MySqlAsyncClient {
            pool: None
        };
        
        match mysql_client.connect(db_url).await {
            Ok(_) => (),
            Err(err) => {
                error!(" Failed to connect to MySQL: {}", err);
                return Err(err)
            }
        };
        
        Ok(mysql_client)
    }
    

    /*
       Function responsible for connecting to MySQL
    */
    async fn connect(&mut self, db_url: &str) -> Result<(), anyhow::Error> {

        let opts = mysql_async::Opts::from_url(db_url)?;
        let conn_pool = mysql_async::Pool::new(opts);

        // Set a 3-second timeout for the connection attempt
        match timeout(Duration::from_secs(5), conn_pool.get_conn()).await {
            Ok(Ok(conn)) => {
                drop(conn);  // Return the connection to the pool
                self.pool = Some(conn_pool);
                info!("MySQL Connection POOL creation was successful");
                Ok(())
            },
            Ok(Err(e)) => Err(anyhow!("Failed to get connection: {}", e)),
            Err(_) => Err(anyhow!("Connection attempt timed out")),
        }
    }

    /*
        Functions that import the connection from the MySQL Connection pool.
    */
    async fn get_conn_from_pool(&self) -> Result<mysql_async::Conn, anyhow::Error> {
        
        let self_db_pool = match &self.pool {
            Some(self_db_pool) => self_db_pool,
            None => return Err(anyhow::Error::msg("Option was None"))
        };
        
        let conn = self_db_pool.get_conn().await?;

        Ok(conn)
    }


    /*
        Dynamic query execution function.
    */
    pub async fn query_select_from_param<T,P>(&self, query: &str, params: P) -> Result<Vec<T>, anyhow::Error> where T: FromRow + std::marker::Send , P:Into<mysql_async::Params> + std::marker::Send {

        let mut conn = self.get_conn_from_pool().await?;
        
        let query_res = conn.exec_map(query, params, T::from_row).await?;

        Ok(query_res)
    }

}