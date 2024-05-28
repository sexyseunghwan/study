use crate::common::*;
use crate::utils::*;
use crate::es_helper::*;
use crate::data_obj::*;
use crate::mysqlconn::*;
use crate::kafka_helper::*;
// use crate::tele_helper::*;



/*
    Function that extracts a list containing ES cluster information that is the target of scheduling.
*/
fn get_es_cluster_info_list(mysql_client: &MySqlClient, aes_infos: AesInfos) -> Result<Vec<NosqlScheduleInfoExtend>, anyhow::Error> {
    
    // The final information list for the Es cluster to be scheduled.
    let mut nosql_cluster_infos: Vec<NosqlScheduleInfoExtend> = Vec::new();
    
    // Information list of Es clusters to be scheduled.
    let target_es_cluster_infos = mysql_client.query_nosql_cluster_info()?;
    
    for es_cluster in target_es_cluster_infos {
        
        // List of host information for the Es cluster to be scheduled.
        let target_host_infos = mysql_client.query_nosql_host_infos(es_cluster.cluster_name())?;
        
        // Final information of the Es cluster to be scheduled.
        let extend_es_cluster_infos = NosqlScheduleInfoExtend::new(&es_cluster, target_host_infos, &aes_infos)?;
        
        nosql_cluster_infos.push(extend_es_cluster_infos);
    }

    Ok(nosql_cluster_infos)
}

/*
    Function that queries index pattern information with log retention period set for a specific ES cluster.
*/
async fn get_to_delete_index_pattern_list(mysql_client: &MySqlClient, cluster_name: &str) -> Result<Vec<IndexPatternInfo>, anyhow::Error> {
    
    // Query index scheduling information that exists in the ES-cluster.
    Ok(mysql_client.query_nosql_delete_index_pattern_infos(cluster_name)?)
}


/*
    Function to send a warning alarm to a specific topic in Kafka
*/
async fn send_message_to_kafka_alarm<T: AlarmDetail>(mon_metric_form: &AlarmMetricForm<T>, kafka_client: &ProduceBroker, topic_name: &str) -> Result<(), anyhow::Error> {

    let mon_metric_form = serde_json::to_string(&mon_metric_form)?;
    let mon_metric_form_str = mon_metric_form.as_str();
    
    match kafka_client.produce_message(topic_name, mon_metric_form_str).await {
        Ok(_) => (),
        Err(err) => {
            return Err(anyhow!(err.to_string()));
        }
    }
    
    Ok(())
}


/*
    Function that REMOVES all indices belonging to an INDEX PATTERN.
*/
async fn delete_index_belong_pattern(kafka_client: &ProduceBroker, target_es_conn: &EsHelper, cluster_name: &str, index_patt_list: Vec<IndexPatternInfo>) -> Result<(), anyhow::Error> {
    
    /*
        Calculates the date by subtracting the log storage period specified in the INDEX PATTERN from today's date.
    */
    // current datetime
    let today_date = match get_curr_utc_time() {
        Ok(today_date) => today_date,
        Err(err) => {
            return Err(anyhow!("{:?}", err));
        }
    };

    // The date pattern to find.
    let date_regex = Regex::new(r"\d{4}\.\d{2}\.\d{2}")?;
    
    for index_patt in index_patt_list {

        let final_loggind_date = get_calculate_time(today_date, *index_patt.presv_period());

        /* 
            Logic that QUERIES the names of all indexes belonging to the INDEX PATTERN.
            ex) nd-partner-log.2024.01.09\nnd-partner-log.2024.01.10\nnd-partner-log.2024.01.11
        */
        let end_point_format = format!("indices/{}*?h=index",index_patt.index_pattern());
        let es_cat_res = target_es_conn.es_cat(end_point_format.as_str(), 5).await?;
        
        
        /* 
            nd-partner-log.2024.01.09\nnd-partner-log.2024.01.10\nnd-partner-log.2024.01.11
            => [Convert]
            index_vec = [nd-partner-log.2024.01.09, nd-partner-log.2024.01.10, nd-partner-log.2024.01.11]
        */
        let index_pattern_vec: Vec<&str> = es_cat_res.split('\n').filter(|&s| !s.is_empty()).collect();
        
        for index_name in index_pattern_vec {

            if let Some(date_match) = date_regex.find(index_name) {
                
                let date_str = date_match.as_str();
                let parsed_data =  NaiveDate::parse_from_str(date_str, "%Y.%m.%d")?;

                if parsed_data < final_loggind_date {
                    
                    // The task of removing the matching index.
                    /*
                        ** ======================== [Warning] ======================== **
                        Indexes whose retention period has expired are REMOVED.
                    */
                    // ==========================DELETE CODE==================================
                    let delete_res = target_es_conn.es_delete(index_name, 5).await?;
                    let resp_json: Value = serde_json::from_value(delete_res)?;
                    
                    if resp_json.get("error").is_some() {
                        // if index removal fails
                        let error_resp: Value = serde_json::from_value(resp_json)?;
                        error!("{}", error_resp);

                        let alarm_error_list = vec![AlarmDetailError::new(error_resp.to_string())];
                        let detail_infos: AlarmMetricForm<AlarmDetailError> = AlarmMetricForm::new(String::from("error_alarm"), String::from("ES"), cluster_name.to_string(), alarm_error_list);
                
                        send_message_to_kafka_alarm(&detail_infos, kafka_client, "nosql_mon_log").await?;
                        
                    } else {
                        // If index removal succeeds
                        let success_response: Value = serde_json::from_value(resp_json)?;
                        let msg_log = format!("[{}] {} index was successfully deleted. {}", cluster_name, index_name, success_response);
                        info!("{}", msg_log);
                    }
                } 
            }
        }
    }
    
    Ok(())
}


/*
    Controller
*/
pub async fn controller() {

    // Select compilation environment
    dotenv().ok();
    let c_version = env::var("COMPILE_ENV").expect("Compile type must be set");
    
    // Monitoring MySQL connection object
    let mysql_client = match MySqlClient::new(&c_version) {
        Ok(mysql_client) => mysql_client,
        Err(err) => {
            error!("Failed to create Elasticsearch client: {:?}", err);
            return;
        }
    };
    
    // Get AES infos
    let aes_infos = match get_aes_infos("./data_file/enc/enc_info.json") {
        Ok(aes_infos) => aes_infos,
        Err(err) => {
            error!("Failed to convert AesInfos object: {:?}", err);
            panic!("Failed to convert AesInfos object: {:?}", err);
        }
    };
    
    // NoSQL cluster information included in index scheduling.
    let nosql_infos = match get_es_cluster_info_list(&mysql_client, aes_infos) {
        Ok(res) => res,
        Err(err) => {
            error!("Failed to : {:?}", err);
            panic!("{}", err);
        }   
    };

    // Kafka connection -> Producer Connection
    let kafka_client = match ProduceBroker::new(&c_version) {
        Ok(kafka_client) => kafka_client,
        Err(err) => {
            error!("Failed to create Kafka client: {:?}", err);
            panic!("Failed to create Kafka client: {:?}", err)
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
        match delete_index_belong_pattern(&kafka_client, &target_es_conn, nosql_obj.cluster_name(), to_delete_index_patt_list).await {
            Ok(_) => (),
            Err(err) => {
                error!("Failed to erase indexes belonging to index patterns. : {:?}", err);
                continue;
            }
        }
    }
}