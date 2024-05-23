use crate::common::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct MySqlDieselInfo {
    ip_address: String,
    port: u16,
    database_name: String,
    username: String,
    password: String
}


/*
    Function to initialize sql connection pool globally - Use Diesel Crate 
*/
pub fn init_rdb_diesel_conn() -> Result<diesel::r2d2::Pool<ConnectionManager<MysqlConnection>>, anyhow::Error> {

    let database_url = env::var("DATABASE_URL").expect("'DATABASE_URL' must be set");
    
    let conn_manager = ConnectionManager::<MysqlConnection>::new(database_url);
    let conn_pool: diesel::r2d2::Pool<ConnectionManager<MysqlConnection>> = diesel::r2d2::Pool::builder().build(conn_manager)?;
    
    Ok(conn_pool)
}



#[derive(Debug)]
pub struct MySqlDieselClient {
    pub db_pool: diesel::r2d2::Pool<ConnectionManager<MysqlConnection>>
}

impl MySqlDieselClient {


    /*
        NEW Constructor
    */
    pub fn new(db_pool: diesel::r2d2::Pool<ConnectionManager<MysqlConnection>>) -> Self {
        
        MySqlDieselClient { db_pool }
    }
    
    /*
        Function that import the connection from the MySQL Connection pool.
    */
    pub async fn get_conn_from_diesel_conn(&self) -> Result<diesel::r2d2::PooledConnection<ConnectionManager<MysqlConnection>>, anyhow::Error> {
        
        let self_db_pool = self.db_pool.clone();

        let diesel_conn: diesel::r2d2::PooledConnection<ConnectionManager<MysqlConnection>> = task::spawn_blocking(move || self_db_pool.get())
            .await
            .map_err(|e| anyhow::Error::new(e))?
            .map_err(|e| anyhow::Error::new(e))?;
        
        Ok(diesel_conn)
    }   

}