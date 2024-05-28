use crate::common::*;
use crate::data_obj::*;

use log::{error, info};

pub struct MySqlClient {
    pub pool: Option<Pool>
}

#[derive(Serialize, Deserialize, Debug)]
struct MySqlInfo {
    pub ip_address: String,
    pub port: u16,
    pub database_name: String,
    pub username: String,
    pub password: String
}

impl MySqlClient {
    
    /*
        Constructor of MySqlClient
    */
    pub fn new(c_version: &str) -> Result<Self, mysql::Error> {
        
        let mysql_file_path = if c_version == "local" {
            "./data_file/mysql_info/sql_info_dev.json"
        } else if c_version == "test" {
            "./data_file/mysql_info/sql_info_devs.json"
        } else {
            "./data_file/mysql_info/sql_info_prod.json"
        };
        
        let parse_sql_data = fs::read_to_string(mysql_file_path).expect("An error occurred while opening the mysql information file."); 
        let mysql_info_json: MySqlInfo = serde_json::from_str(&parse_sql_data).expect("An error occurred while parsing the mysql information file json information.");

        let mut mysql_client = MySqlClient {
            pool: None
        };

        match mysql_client.connect(mysql_info_json.username, mysql_info_json.password, mysql_info_json.ip_address, mysql_info_json.port, mysql_info_json.database_name) {
            Ok(_) => (),
            Err(err) => {
                error!("Failed to connect to MySQL: {}", err);
                return Err(err);
            }
        };
        

        Ok(mysql_client)
    }


    /*
       Function responsible for connecting to MySQL
    */
    pub fn connect(&mut self, username:String, password: String, ip_address:String, port:u16, database_name:String) -> Result<(), mysql::Error> {
        
        let database_url = format!(
            "mysql://{}:{}@{}:{}/{}",
            username, password, ip_address, port, database_name
        );
        
        let opts_url = Opts::from_url(&database_url)?;
        let conn_pool = Pool::new(opts_url)?;
        
        self.pool = Some(conn_pool);
        
        info!("MySQL Connection POOL creation was successful");
        
        Ok(())
    }
    
    /*
        Function that QUERIES data information in the NOSQL_CLUSTER_TYPE table.
    */
    pub fn query_nosql_cluster_info(&self) -> Result<Vec<NosqlScheduleInfo>, mysql::Error> {
        
        let pool = self.pool.as_ref().expect("Pool has not been initialized");

        let mut conn = match pool.get_conn() {
            Ok(my_conn) => my_conn,
            Err(err) => {
                error!("{}",err);
                return Err(err);
            }
        };
        
        let conn_query = r"
                SELECT
                    nct.cluster_name
                ,	MAX(nct.system_type) as system_type
                ,   MAX(nct.user_id) as user_id
                ,	MAX(nct.user_pw_enc) as user_pw_enc
                ,	MAX(nct.system_version)	as system_version
                ,	MAX(nct.ssl_option) as ssl_option
                FROM NOSQL_CLUSTER_TYPES nct
                INNER JOIN NOSQL_INDEX_SCHEDULE nis ON nct.cluster_name = nis.cluster_name
                WHERE nct.system_type = 'ES'
                GROUP BY nct.cluster_name";
        
        let selected_nosql_clusters: Vec<NosqlScheduleInfo> = conn.query_map(
            conn_query,
            |(cluster_name, system_type, user_id, user_pw_enc, system_version, ssl_option)| {
                NosqlScheduleInfo::new(cluster_name, system_type, user_id, user_pw_enc, system_version, ssl_option)
            },
        )?;

        Ok(selected_nosql_clusters)

    }
    


    /*
        Function that QUERIES data information in the NOSQL_HOST_INFOS table.
    */
    pub fn query_nosql_host_infos(&self, cluster_name: &str) -> Result<Vec<String>, mysql::Error> {

        let pool = self.pool.as_ref().expect("Pool has not been initialized");

        let mut conn = match pool.get_conn() {
            Ok(my_conn) => my_conn,
            Err(err) => {
                error!("{}",err);
                return Err(err);
            }
        };
        
        let query = r"
            SELECT
                CONCAT(host_ip, ':', host_port) as host_info
            FROM NOSQL_HOST_INFO
            WHERE cluster_name = ?
            AND system_type = 'ES'";
        
        let params = (cluster_name,);

        let cluster_host_info = match conn.exec_map(
            query,
            params,
            | host_info| {
                host_info 
            },
        )
        {
            Ok(res_vec) => res_vec,
            Err(err) => {
                error!("{}",err);
                return Err(err);
            }
        }; 
        
        Ok(cluster_host_info)
    }

    

    /*
        Function that returns a list of index patterns to be scheduled included in a specific cluster.
    */
    pub fn query_nosql_delete_index_pattern_infos(&self, cluster_name: &str) -> Result<Vec<IndexPatternInfo>, mysql::Error> {
        
        let pool = self.pool.as_ref().expect("Pool has not been initialized");

        let mut conn = match pool.get_conn() {
            Ok(my_conn) => my_conn,
            Err(err) => {
                error!("{}",err);
                return Err(err);
            }
        };
        
        let query = r"
            SELECT 
                index_pattern
            ,	presv_period
            FROM NOSQL_INDEX_SCHEDULE
            WHERE cluster_name = ?
            AND system_type = 'ES'";
        
        let params = (cluster_name,);

        let index_pattern_info = match conn.exec_map(
            query,
            params,
            | (index_pattern, presv_period)| {
                IndexPatternInfo::new(index_pattern, presv_period) 
            },
        )
        {
            Ok(res_vec) => res_vec,
            Err(err) => {
                error!("{}",err);
                return Err(err);
            }
        }; 

        Ok(index_pattern_info)

    }

}