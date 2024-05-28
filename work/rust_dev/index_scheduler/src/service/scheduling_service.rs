use crate::common::*;

use crate::service::mysql_async_service::*;
use crate::service::es_service::*;
use crate::service::kafka_service::*;

use crate::utils_module::date_utils::*;

use crate::dtos::data_obj::*;



/*
    Function that extracts a list containing ES cluster information that is the target of scheduling.
*/
pub async fn get_es_cluster_info_list(mysql_client: &MySqlAsyncClient, aes_infos: AesInfos) -> Result<Vec<NosqlScheduleInfoExtend>, anyhow::Error> {
    
    // The final information list for the Es cluster to be scheduled.
    let mut nosql_cluster_infos: Vec<NosqlScheduleInfoExtend> = Vec::new();
    
    // Information list of Es clusters to be scheduled.
    let target_es_cluster_infos: Vec<NosqlScheduleInfo> = 
        mysql_client.query_select_from_param(
        r"
            SELECT
                nct.cluster_name
            ,	MAX(nct.system_type) as system_type
            ,   IFNULL((MAX(nct.kibana_url)), 'NONE') AS kibana_url
            ,   MAX(nct.user_id) as user_id
            ,	MAX(nct.user_pw_enc) as user_pw_enc
            ,	MAX(nct.system_version)	as system_version
            ,	MAX(nct.ssl_option) as ssl_option
            FROM NOSQL_CLUSTER_TYPES nct
            INNER JOIN NOSQL_INDEX_SCHEDULE nis ON nct.cluster_name = nis.cluster_name
            WHERE nct.system_type = ?
            GROUP BY nct.cluster_name"
            , ("ES", )).await?;
    

    for es_cluster in target_es_cluster_infos {
        
        // List of host information for the Es cluster to be scheduled.
        let target_host_infos = 
            mysql_client.query_select_from_param(
            r"
            SELECT
                CONCAT(host_ip, ':', host_port) as host_info
            FROM NOSQL_HOST_INFO
            WHERE cluster_name = ?
            AND system_type = ?"
            , (es_cluster.cluster_name(),"ES", )).await?;

        // Final information of the Es cluster to be scheduled.
        let extend_es_cluster_infos = NosqlScheduleInfoExtend::new(&es_cluster, target_host_infos, &aes_infos)?;
        
        nosql_cluster_infos.push(extend_es_cluster_infos);
    }

    Ok(nosql_cluster_infos)
}


/*
    List of index patterns targeted for deletion scheduling
*/
pub async fn get_to_delete_index_pattern_list(mysql_client: &MySqlAsyncClient, cluster_name: &str) -> Result<Vec<IndexPatternInfo>, anyhow::Error> {
    
    // Query index scheduling information that exists in the ES-cluster.
    let index_pattern_list: Vec<IndexPatternInfo> = 
        mysql_client.query_select_from_param(
        r"
            SELECT
                index_pattern
            ,	presv_period
            FROM NOSQL_INDEX_SCHEDULE
            WHERE cluster_name = ?
            AND system_type = ?"
            , (cluster_name, "ES", )
        ).await?;

    Ok(index_pattern_list)
}



/*
    Function that REMOVES all indices belonging to an INDEX PATTERN.
*/
pub async fn delete_index_belong_pattern(kafka_client: &ProduceBroker, target_es_conn: &EsHelper, cluster_name: &str, kibana_url: &str, index_patt_list: Vec<IndexPatternInfo>) -> Result<(), anyhow::Error> {
    
    /*
        Calculates the date by subtracting the log storage period specified in the INDEX PATTERN from today's date.
    */
    // current datetime
    let today_date = get_curr_utc_time()?;

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
                        let alarm_error_list = vec![AlarmDetailError::new(format!("Failed to clear '{}' index. : {}", index_name, error_resp.to_string()))];
                        let alarm_detail_infos: AlarmMetricForm<AlarmDetailError> = AlarmMetricForm::new(String::from("error_alarm"), String::from("ES"), cluster_name.to_string(), kibana_url.to_string(),alarm_error_list);
                        kafka_client.send_message_to_kafka_alarm(&alarm_detail_infos, "nosql_mon_log").await?;
                        
                        // It records the error log. - ERROR LOG
                        let log_detail = LogDetail::new(index_name.to_string(), false, error_resp.to_string());
                        kafka_client.send_message_to_kafka_log(&log_detail, "index_schedule_log").await?;
                        
                    } else {
                        // If index removal succeeds
                        let success_response: Value = serde_json::from_value(resp_json)?;
                        let msg_log = format!("[{}] {} index was successfully deleted. {}", cluster_name, index_name, success_response);
                        info!("{}", msg_log);
                        
                        // It records the error log. - SUCCESS LOG
                        let log_detail = LogDetail::new(index_name.to_string(), true, msg_log);
                        kafka_client.send_message_to_kafka_log(&log_detail, "index_schedule_log").await?;
                    }
                } 
            }
        }
    }
    
    Ok(())
}