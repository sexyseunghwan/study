use crate::common::*;

use crate::service::es_service::*;
use crate::service::kafka_service::*;
use crate::service::mysql_async_service::*;
use crate::service::scheduling_service::*;

use crate::utils_module::crypt_utils::*;


/*
    Main controller
*/
pub async fn main_controller() {
    
    // Select compilation environment
    dotenv().ok();
    let rdb_url = env::var("RDB_URL").expect("'DB_URL' must be set");
    let kafka_host = env::var("KAFKA_HOST").expect("'KAFKA_HOST' must be set");
    let aes_key = env::var("AES_KEY").expect("'AES_KEY' must be set");
    let aes_iv = env::var("AES_IV").expect("'AES_IV' must be set");
    
    // MySQL connection
    let mysql_client: MySqlAsyncClient = match MySqlAsyncClient::new(&rdb_url).await {
        Ok(mysql_client) => mysql_client,
        Err(err) => {
            error!("Failed to create mysql client: {:?}", err);
            panic!("Failed to create mysql client: {:?}", err);
        }
    };
    
    // Get AES infos
    let aes_infos = match get_aes_infos(&aes_key, &aes_iv) {
        Ok(aes_infos) => aes_infos,
        Err(err) => {
            error!("Failed to convert AesInfos object: {:?}", err);
            panic!("Failed to convert AesInfos object: {:?}", err);
        }
    };
    
    // Kafka connection -> Producer Connection
    let kafka_client = match ProduceBroker::new(&kafka_host) {
        Ok(kafka_client) => kafka_client,
        Err(err) => {
            error!("Failed to create Kafka client: {:?}", err);
            panic!("Failed to create Kafka client: {:?}", err)
        }
    };
    
    // NoSQL cluster information included in index scheduling.
    let nosql_infos = match get_es_cluster_info_list(&mysql_client, aes_infos).await {
        Ok(res) => res,
        Err(err) => {
            error!("Failed to : {:?}", err);
            panic!("{}", err);
        }   
    };
    
    /*
        Executes the INDEX DELETION SCHEDULER while traversing the nosql cluster information included in index scheduling.
    */ 
    for nosql_obj in nosql_infos {
        
        // ES connection object subject to scheduling
        let target_es_conn = match EsHelper::new(&nosql_obj).await {
            Ok(target_es_conn) => target_es_conn,
            Err(err) => {
                error!("Failed to create Elasticsearch client: {:?}", err);
                continue;
            }
        };
        
        // List of index patterns targeted for deletion scheduling
        let to_delete_index_patt_list = match get_to_delete_index_pattern_list(&mysql_client, nosql_obj.cluster_name()).await {
            Ok(to_delete_index_patt_list) => to_delete_index_patt_list,
            Err(err) => {
                error!("Failed to get index pattern list that you are trying to erase. : {:?}", err);
                continue;
            }
        };
        
        /* COLLECTS all index patterns with the selected lifecycle on the target ES cluster and REMOVES out-of-lifecycle indexes. */
        match delete_index_belong_pattern(&kafka_client, &target_es_conn, nosql_obj.cluster_name(), nosql_obj.kibana_url(), to_delete_index_patt_list).await {
            Ok(_) => (),
            Err(err) => {
                error!("Failed to erase indexes belonging to index patterns. : {:?}", err);
                continue;
            }
        }

    }


}