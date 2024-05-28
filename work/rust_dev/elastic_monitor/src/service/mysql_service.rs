// ==================================== deprecated ====================================
// use crate::common::*;


// #[derive(Debug)]
// pub struct MySqlClient {
//     pool: Option<Pool>
// }

// #[derive(Serialize, Deserialize, Debug)]
// struct MySqlInfo {
//     ip_address: String,
//     port: u16,
//     username: String,
//     password: String,
//     database_name: String
// }

// impl MySqlClient {

//     /*
//         Constructor of MySqlClient
//     */
//     pub fn new(c_version: &str) -> Result<Self, mysql::Error> {
        
//         let mysql_file_path = if c_version == "test" {
//             "./data_file/rdb_info/mysql_conn_dev.json"
//         } else if c_version == "local" {
//             "./data_file/rdb_info/mysql_conn_dev_local.json"
//         } else {
//             "./data_file/rdb_info/mysql_conn_prod.json"
//         };
        
//         let parse_sql_data = fs::read_to_string(mysql_file_path).expect("An error occurred while opening the mysql information file."); 
//         let mysql_info_json: MySqlInfo = serde_json::from_str(&parse_sql_data).expect("An error occurred while parsing the mysql information file json information.");
        
//         let mut mysql_client = MySqlClient {
//             pool: None
//         };
        
//         match mysql_client.connect(mysql_info_json.username, mysql_info_json.password, mysql_info_json.ip_address, mysql_info_json.port, mysql_info_json.database_name) {
//             Ok(_) => (),
//             Err(err) => {
//                 error!(" Failed to connect to MySQL: {}", err);
//                 return Err(err)
//             }
//         };
        
//         Ok(mysql_client)
//     }
    
    
//     /*
//        Function responsible for connecting to MySQL
//     */
//     fn connect(&mut self, username:String, password: String, ip_address:String, port:u16, database_name:String) -> Result<(), mysql::Error> {
                
//         let database_url = format!(
//             "mysql://{}:{}@{}:{}/{}",
//             username, password, ip_address, port, database_name
//         );
        
//         let opts_url = Opts::from_url(&database_url)?;
//         let conn_pool = Pool::new(opts_url)?;
        
//         self.pool = Some(conn_pool);
        
//         info!("MySQL Connection POOL creation was successful");
        
//         Ok(())
//     }

//     /*
//         Functions that import the connection from the MySQL Connection pool.
//     */
//     fn get_conn_from_pool(&self) -> Result<PooledConn, mysql::Error> {
        
//         let pool = self.pool.as_ref().expect("Pool has not been initialized");

//         let conn = match pool.get_conn() {
//             Ok(my_conn) => my_conn,
//             Err(err) => {
//                 error!("{}",err);
//                 return Err(err);
//             }
//         };

//         Ok(conn)
//     }


//     /*
//         Dynamic query execution function.
//     */
//     pub fn query_select_from_param<T,P>(&self, query: &str, params: P) -> Result<Vec<T>, mysql::Error> where T: FromRow, P:Into<Params> {

//         let mut conn = self.get_conn_from_pool()?;
        
//         let query_res = conn.exec_map(query, params, T::from_row)?;

//         Ok(query_res)
//     }

    
//     /*
//         Static query execution function.
//     */
//     pub fn query_select_from<T>(&self, query: &str) -> Result<Vec<T>, mysql::Error> where T: FromRow {

//         let mut conn = self.get_conn_from_pool()?;

//         let query_res = conn.query_map(query, T::from_row)?;

//         Ok(query_res)
//     }

// }